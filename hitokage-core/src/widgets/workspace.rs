use crate::common::Align;
use crate::lua::event::STATE;
use anyhow::Context;
use gtk4::prelude::*;
use gtk4::Constraint;
use gtk4::ConstraintLayout;
use relm4::prelude::*;
use relm4::ComponentParts;
use relm4::ComponentSender;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::mpsc::Sender;
use std::sync::Arc;
use std::sync::Mutex;

type WorkspaceState = (Option<String>, bool, bool);

#[derive(Debug, Clone)]
pub enum WorkspaceMsgHook {
  GetHalign(Sender<Align>),
  SetHalign(Align),
  GetItemHeight(Sender<u32>),
  SetItemHeight(u32),
  GetItemWidth(Sender<u32>),
  SetItemWidth(u32),
}

#[derive(Debug, Clone)]
pub enum WorkspaceMsg {
  Workspaces(Vec<WorkspaceState>),
  FocusWorkspace(usize),
  LuaHook(WorkspaceMsgHook),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct WorkspaceProps {
  item_width: Option<u32>,
  item_height: Option<u32>,
  halign: Option<Align>,
}

pub struct Workspace {
  flowbox: gtk4::FlowBox,
  id: u32, // win id
  // workspaces: Vec<WorkspaceState>,
  constraint_layout: ConstraintLayout,
  workspaces_to_check_constraints: Arc<Mutex<HashMap<i32, Vec<Constraint>>>>, // this maps a workspace id to the constraints that should be reevaluated every workspace change
  
  // lua accessible
  item_width: i32,
  item_height: i32,
  halign: Align,
}

#[relm4::component(pub)]
impl Component for Workspace {
  type Input = WorkspaceMsg;
  type Output = ();
  type Init = (WorkspaceProps, u32); // win id
  type CommandOutput = ();

  view! {
    #[root]
    gtk::Box {
      set_hexpand: true,
      set_halign: gtk4::Align::Start,
      #[name="flowbox"]
      gtk::FlowBox {
        set_height_request: 16,
        set_hexpand: true,
        set_vexpand: true,
      },
    },
  }

  fn init(propst: Self::Init, root: Self::Root, sender: ComponentSender<Self>) -> ComponentParts<Self> {
    let (props, id) = propst;

    let widgets = view_output!();
    let flowbox = widgets.flowbox.clone();

    {
      let sender = sender.clone();
      flowbox.connect_selected_children_changed(move |root| {
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
    flowbox.set_layout_manager(Some(constraint_layout.clone()));

    let item_width = props.item_width.unwrap_or(16) as i32;
    let item_height = props.item_height.unwrap_or(16) as i32;
    let halign =  props.halign.unwrap_or(Align::Start);

    root.set_halign(halign.clone().into());

    let model = Workspace {
      flowbox: flowbox.clone(),
      id,
      constraint_layout: constraint_layout,
      workspaces_to_check_constraints: Arc::new(Mutex::new(HashMap::new())),
      item_width,
      item_height,
      halign,
    };

    // let (relm4s, relm4r) = relm4::channel::<()>();
    STATE.subscribe(sender.input_sender(), move |state| {
      // we only care about the most recent state
      let workspaces = get_workspaces(&state.clone(), id);

      WorkspaceMsg::Workspaces(workspaces.unwrap())
    });

    ComponentParts { model, widgets }
  }

  fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>, root: &Self::Root) {
    match msg {
      WorkspaceMsg::Workspaces(workspaces) => {
        update_workspaces(
          &self.flowbox,
          &workspaces,
          &self.constraint_layout,
          Arc::clone(&self.workspaces_to_check_constraints),
          self.item_width,
          self.item_height,
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
      WorkspaceMsg::LuaHook(hook) => match hook {
        WorkspaceMsgHook::GetHalign(tx) => {
          tx.send(self.halign).unwrap()
        }
        WorkspaceMsgHook::SetHalign(halign) => {
          root.set_halign(halign.clone().into());
          self.halign = halign
        }
        WorkspaceMsgHook::GetItemHeight(tx) => {
          tx.send(self.item_width as u32).unwrap();
        }
        WorkspaceMsgHook::SetItemHeight(item_height) => {
          self.item_width = item_height as i32;
          let state = STATE.read();
          let workspaces = get_workspaces(&state.clone(), self.id).unwrap();
          update_workspaces(
            &self.flowbox,
            &workspaces,
            &self.constraint_layout,
            Arc::clone(&self.workspaces_to_check_constraints),
            self.item_width,
            self.item_height,
          );
        }
        WorkspaceMsgHook::GetItemWidth(tx) => {
          tx.send(self.item_width as u32).unwrap();
        }
        WorkspaceMsgHook::SetItemWidth(item_width) => {
          self.item_width = item_width as i32;
          let state = STATE.read();
          let workspaces = get_workspaces(&state.clone(), self.id).unwrap();
          update_workspaces(
            &self.flowbox,
            &workspaces,
            &self.constraint_layout,
            Arc::clone(&self.workspaces_to_check_constraints),
            self.item_width,
            self.item_height,
          );
        }
      },
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
  flowbox: &gtk4::FlowBox,
  workspaces: &Vec<WorkspaceState>,
  constraints_layout: &ConstraintLayout,
  workspaces_to_check_constraints_guard: Arc<Mutex<HashMap<i32, Vec<Constraint>>>>,
  width: i32,
  height: i32,
) -> () {
  let mut i = 0;
  loop {
    match flowbox.child_at_index(i as i32) {
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
              flowbox.select_child(&child);
            }
            (name, false, is_visible) => {
              child
                .first_child()
                .unwrap()
                .downcast::<gtk4::Label>()
                .unwrap()
                .set_label(&name.clone().unwrap_or((i + 1).to_string()));
              child.set_visible(*is_visible);
              flowbox.unselect_child(&child);
            }
          },
          None => flowbox.remove(&child),
        };
      }
      None => match workspaces.get(i) {
        Some((name, is_focused, _)) => {
          let label = gtk::Label::new(Some(&name.clone().unwrap_or((i + 1).to_string())));
          flowbox.append(&label);
          let child = &flowbox.child_at_index(i as i32).unwrap();
          child.set_hexpand(false);
          if *is_focused {
            flowbox.select_child(child);
          }
          child.set_size_request(width, height);
        }
        None => break,
      },
    }

    if let Some(child) = flowbox.child_at_index(i as i32) {
      let mut workspaces_to_check_constraints = workspaces_to_check_constraints_guard.lock().unwrap();

      if let Some(constraints) = workspaces_to_check_constraints.get_mut(&(i as i32)) {
        for constraint in constraints.drain(..) {
          // log::debug!("[Workspace] removing constraint {:?}", constraint);
          constraints_layout.remove_constraint(&constraint);
        }
      }

      let mut to_check_constraints: Vec<Constraint> = Vec::new();

      let first_visible_or_focused = workspaces
        .iter()
        .position(|(_, is_focused, is_visible)| *is_visible || (!*is_visible && *is_focused));

      if let Some(first) = first_visible_or_focused {
        if i == first {
          let constraint = Constraint::new(
            Some(&child),
            gtk4::ConstraintAttribute::Start,
            gtk4::ConstraintRelation::Eq,
            Some(flowbox),
            gtk4::ConstraintAttribute::Start,
            1.0,
            0.0,
            1000000000,
          );

          to_check_constraints.push(constraint);
        }
      }

      let prev_visible = find_first_visible_previous_sibling(&gtk4::Widget::from(child.clone()));

      if i != 0 {
        if let Some(prev) = prev_visible {
          let constraint = Constraint::new(
            Some(&child),
            gtk4::ConstraintAttribute::Start,
            gtk4::ConstraintRelation::Eq,
            Some(&prev),
            gtk4::ConstraintAttribute::End,
            1.0,
            0.0,
            1001001000,
          );

          to_check_constraints.push(constraint);
        }
      }

      let last_visible_or_focused = workspaces
        .iter()
        .rposition(|(_, is_focused, is_visible)| *is_visible || (!*is_visible && *is_focused));

      if let Some(last) = last_visible_or_focused {
        if i == last {
          let constraint = Constraint::new(
            Some(&child),
            gtk4::ConstraintAttribute::End,
            gtk4::ConstraintRelation::Eq,
            Some(flowbox),
            gtk4::ConstraintAttribute::End,
            1.0,
            0.0,
            1000000000,
          );

          to_check_constraints.push(constraint);
        }
      }

      for constraint in &to_check_constraints {
        // log::debug!("[Workspace] adding constraint {:?}", constraint);
        constraints_layout.add_constraint(constraint.clone());
      }

      workspaces_to_check_constraints.insert(i as i32, to_check_constraints);
    }

    i += 1;
  }

  flowbox.show();

  ()
}

fn find_first_visible_previous_sibling(child: &gtk4::Widget) -> Option<gtk4::Widget> {
  let mut prev_sibling = child.prev_sibling();
  while let Some(ref sibling) = prev_sibling {
    if sibling.is_visible() {
      return Some(sibling.clone());
    }
    prev_sibling = sibling.prev_sibling();
  }
  None
}
