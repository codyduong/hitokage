use crate::lua::event::STATE;
use anyhow::Context;
use gtk4::prelude::*;
use gtk4::Constraint;
use gtk4::ConstraintLayout;
use relm4::prelude::*;
use relm4::ComponentParts;
use relm4::ComponentSender;
use relm4::SimpleComponent;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use std::sync::Mutex;

type WorkspaceState = (Option<String>, bool, bool);

#[derive(Debug, Clone)]
pub enum WorkspaceMsg {
  Workspaces(Vec<WorkspaceState>),
  FocusWorkspace(usize),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct WorkspaceProps {
  width: Option<i32>,
  height: Option<i32>,
}

pub struct Workspace {
  root: gtk4::FlowBox,
  id: u32, // win id
  // workspaces: Vec<WorkspaceState>,
  constraint_layout: ConstraintLayout,
  workspaces_to_check_constraints: Arc<Mutex<HashMap<i32, Vec<Constraint>>>>, // this maps a workspace id to the constraints that should be reevaluated every workspace change
  width: i32,
  height: i32,
}

#[relm4::component(pub)]
impl SimpleComponent for Workspace {
  type Input = WorkspaceMsg;
  type Output = ();
  type Init = (WorkspaceProps, u32); // win id

  view! {
    #[root]
    gtk::FlowBox {
      set_height_request: 16,
      set_hexpand: false,
      set_vexpand: true,
    }
  }

  fn init(propst: Self::Init, root: Self::Root, sender: ComponentSender<Self>) -> ComponentParts<Self> {
    let (props, id) = propst;

    {
      let sender = sender.clone();
      root.connect_selected_children_changed(move |root| {
        if let Some(child) = root
          .clone()
          .downcast::<gtk::FlowBox>()
          .unwrap()
          .selected_children()
          .first()
        {
          let workspace_index = child.index();

          sender.input(WorkspaceMsg::FocusWorkspace(workspace_index as usize))
        }

        ()
      });
    }

    let constraint_layout = ConstraintLayout::new();

    let model = Workspace {
      root: root.clone(),
      id,
      constraint_layout: constraint_layout.clone(),
      workspaces_to_check_constraints: Arc::new(Mutex::new(HashMap::new())),
      width: props.width.unwrap_or(16),
      height: props.height.unwrap_or(16),
    };

    let widgets = view_output!();

    root.set_layout_manager(Some(constraint_layout));

    // let (relm4s, relm4r) = relm4::channel::<()>();
    STATE.subscribe(sender.input_sender(), move |state| {
      // we only care about the most recent state
      let workspaces = get_workspaces(&state.clone(), id);

      WorkspaceMsg::Workspaces(workspaces.unwrap())
    });

    ComponentParts { model, widgets }
  }

  fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>) {
    match msg {
      WorkspaceMsg::Workspaces(workspaces) => {
        update_workspaces(
          &self.root,
          &workspaces,
          &self.constraint_layout,
          Arc::clone(&self.workspaces_to_check_constraints),
          self.width,
          self.height,
        );
      }
      WorkspaceMsg::FocusWorkspace(i) => {
        let state = STATE.read();
        if let Some((workspace_index, _)) = get_workspaces(&state, self.id)
          .unwrap()
          .iter()
          .enumerate()
          .find(|(_, workspace)| workspace.1 == true)
        {
          if workspace_index != i {
            log::info!("hitokage is focusing workspace {}", i);
            let _ = komorebi_client::send_message(&komorebi_client::SocketMessage::FocusWorkspaceNumber(i));
          }
        } else {
          log::error!("We failed to find any focused workspace? What happened!")
        }
      }
    }
  }
}

// get workspace from komorebi
fn get_workspaces(state: &serde_json::Value, monitor_id: u32) -> anyhow::Result<Vec<WorkspaceState>> {
  let mut workspaces_vec: Vec<WorkspaceState> = Vec::new();

  let monitors = state
    .get("monitors")
    .context("Missing 'monitors'")?
    .as_object()
    .context("Invalid 'monitors' format")?;

  let elements = monitors
    .get("elements")
    .context("Missing 'elements' in 'monitors'")?
    .as_array()
    .context("Invalid 'elements' format in 'monitors'")?;

  let monitor = elements
    .iter()
    .find(|monitor| {
      monitor
        .as_object()
        .and_then(|v| v.get("id"))
        .and_then(|v| v.as_u64())
        .map(|id| id == monitor_id as u64)
        .unwrap_or(false)
    })
    .context("Monitor with specified ID not found")?;

  let workspaces = monitor.get("workspaces").context("Missing 'workspaces' in monitor")?;

  let elements = workspaces
    .get("elements")
    .context("Missing 'elements' in 'workspaces'")?
    .as_array()
    .context("Invalid 'elements' format in 'workspaces'")?;

  for workspace in elements {
    let name = workspace.get("name").and_then(|v| v.as_str()).map(String::from);

    // check if we actually have anything in the workspace, if not don't show it
    let has_content = workspace
      .get("containers")
      .and_then(|v| v.as_object())
      .and_then(|v| v.get("elements"))
      .and_then(|v| v.as_array())
      .map_or(false, |elements| !elements.is_empty())
      || workspace
        .get("floating_windows")
        .and_then(|v| v.as_array())
        .map_or(false, |floating| !floating.is_empty());

    workspaces_vec.push((name, false, has_content));
  }

  if let Some(focused) = workspaces.get("focused").and_then(|v| v.as_u64()).map(|v| v as usize) {
    if let Some(workspace_possible) = workspaces_vec.get(focused) {
      workspaces_vec[focused] = (workspace_possible.0.clone(), true, true);
    }
  }

  Ok(workspaces_vec)
}

fn update_workspaces(
  root: &gtk4::FlowBox,
  workspaces: &Vec<WorkspaceState>,
  constraints_layout: &ConstraintLayout,
  workspaces_to_check_constraints_guard: Arc<Mutex<HashMap<i32, Vec<Constraint>>>>,
  width: i32,
  height: i32,
) -> () {
  let mut i = 0;
  let mut workspaces_to_check_constraints = workspaces_to_check_constraints_guard.lock().unwrap();
  loop {
    match root.child_at_index(i as i32) {
      Some(child) => {
        match workspaces.get(i) {
          Some(workspace) => match workspace {
            (name, true, _) => {
              child
                .first_child()
                .unwrap()
                .downcast::<gtk4::Label>()
                .unwrap()
                .set_label(&name.clone().unwrap_or((i + 1).to_string()));
              child.set_visible(true);
              root.select_child(&child);
            }
            (name, false, is_visible) => {
              child
                .first_child()
                .unwrap()
                .downcast::<gtk4::Label>()
                .unwrap()
                .set_label(&name.clone().unwrap_or((i + 1).to_string()));
              child.set_visible(*is_visible);
              root.unselect_child(&child);
            }
          },
          None => root.remove(&child),
        };
      }
      None => match workspaces.get(i) {
        Some((name, is_focused, _)) => {
          let label = gtk::Label::new(Some(&name.clone().unwrap_or((i + 1).to_string())));
          root.append(&label);
          let child = &root.child_at_index(i as i32).unwrap();
          if *is_focused {
            root.select_child(child);
          }

          child.set_size_request(width, height);

          // constraints_layout.add_constraint(Constraint::new(
          //   Some(child),
          //   gtk4::ConstraintAttribute::Width,
          //   gtk4::ConstraintRelation::Le,
          //   None::<&gtk4::FlowBox>,
          //   gtk4::ConstraintAttribute::None,
          //   1.0,
          //   width as f64,
          //   1000000000,
          // ));
          // constraints_layout.add_constraint(Constraint::new(
          //   Some(child),
          //   gtk4::ConstraintAttribute::Height,
          //   gtk4::ConstraintRelation::Le,
          //   None::<&gtk4::FlowBox>,
          //   gtk4::ConstraintAttribute::None,
          //   1.0,
          //   height as f64,
          //   1000000000,
          // ));
          // correctly layout at start of flowbox
          constraints_layout.add_constraint(Constraint::new(
            Some(child),
            gtk4::ConstraintAttribute::Start,
            gtk4::ConstraintRelation::Ge,
            Some(root),
            gtk4::ConstraintAttribute::Start,
            1.0,
            0.0,
            1,
          ));

          // constraints_layout.add_constraint(Constraint::new(
          //   Some(child),
          //   gtk4::ConstraintAttribute::End,
          //   gtk4::ConstraintRelation::Ge,
          //   Some(root),
          //   gtk4::ConstraintAttribute::End,
          //   1.0,
          //   0.0,
          //   1,
          // ));
        }
        None => break,
      },
    }

    if i != 0 {
      let child = &root.child_at_index(i as i32);

      if let Some(constraints) = workspaces_to_check_constraints.get_mut(&(i as i32)) {
        for constraint in constraints.drain(..) {
          // TODO @codyduong we remove this constraint and reconstruct it even if it is the same...
          constraints_layout.remove_constraint(&constraint);
          // log::debug!("[Workspace] removing constraint {:?}", constraint);
        }
      }

      if let Some(child) = child {
        // lead 8 pixels from previous sibling
        if i != 0 {
          // find first visible previous sibling
          let prev_visible = loop {
            let mut prev_sibling = child.prev_sibling();
            while let Some(ref sibling) = prev_sibling {
              if sibling.is_visible() {
                break;
              }
              prev_sibling = sibling.prev_sibling();
            }
            break prev_sibling;
          };

          let mut to_check_constraints: Vec<Constraint> = Vec::new();

          if let Some(prev) = prev_visible {
            to_check_constraints.push(Constraint::new(
              Some(child),
              gtk4::ConstraintAttribute::Start,
              gtk4::ConstraintRelation::Eq,
              Some(&prev),
              gtk4::ConstraintAttribute::End,
              1.0,
              0.0,
              1000,
            ))
          } else {
            to_check_constraints.push(Constraint::new(
              Some(child),
              gtk4::ConstraintAttribute::Start,
              gtk4::ConstraintRelation::Eq,
              Some(root),
              gtk4::ConstraintAttribute::Start,
              1.0,
              0.0,
              1000,
            ))
          }

          // if we are the center element then center everything?

          for constraint in to_check_constraints.clone() {
            constraints_layout.add_constraint(constraint.clone());
            // log::debug!("[Workspace] adding constraint {:?}", constraint);
          }

          workspaces_to_check_constraints.insert(i as i32, to_check_constraints);
        }
      }
    }

    i += 1;
  }

  root.show();

  ()
}
