use crate::lua::event::EVENT;
use crate::lua::event::STATE;
use crate::RelmContainerExtManual;
use anyhow::Context;
use gtk4::prelude::*;
use relm4::prelude::*;
use relm4::ComponentParts;
use relm4::ComponentSender;
use relm4::SimpleComponent;
use serde::{Deserialize, Serialize};

type WorkspaceState = (Option<String>, bool, bool);

#[derive(Debug, Clone)]
pub enum WorkspaceMsg {
  Workspaces(Vec<WorkspaceState>),
}

#[derive(Debug, Deserialize, Serialize)]
pub struct WorkspaceProps {}

pub struct Workspace {
  root: gtk4::FlowBox,
  // id: u32, // win id
  // workspaces: Vec<WorkspaceState>,
}

#[relm4::component(pub)]
impl SimpleComponent for Workspace {
  type Input = WorkspaceMsg;
  type Output = ();
  type Init = (WorkspaceProps, u32); // win id

  view! {
    gtk::FlowBox {
      set_height_request: 16,
    }
  }

  fn init(propst: Self::Init, root: Self::Root, sender: ComponentSender<Self>) -> ComponentParts<Self> {
    let (_props, id) = propst;

    let model = Workspace {
      root: root.clone(),
      // id
    };

    let widgets = view_output!();

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
        update_workspaces(&self.root, &workspaces);
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

fn update_workspaces(root: &gtk4::FlowBox, workspaces: &Vec<WorkspaceState>) -> () {
  let mut i = 0;
  let mut prev: Option<gtk4::FlowBoxChild> = None;
  loop {
    match root.child_at_index(i as i32) {
      Some(child) => {
        prev = Some(child.clone());
        match workspaces.get(i) {
          Some(workspace) => match workspace {
            (name, true, _) => {
              child
                .first_child()
                .unwrap()
                .downcast::<gtk4::Label>()
                .unwrap()
                .set_label(&name.clone().unwrap_or(i.to_string()));
              child.set_visible(true);
              root.select_child(&child)
            }
            (name, false, is_visible) => {
              child
                .first_child()
                .unwrap()
                .downcast::<gtk4::Label>()
                .unwrap()
                .set_label(&name.clone().unwrap_or(i.to_string()));
              child.set_visible(*is_visible);
              root.unselect_child(&child)
            }
          },
          None => root.remove(&child),
        };
      }
      None => match workspaces.get(i) {
        Some((name, is_focused, is_visible)) => {
          let label = gtk::Label::new(Some(&name.clone().unwrap_or((i + 1).to_string())));
          let child = gtk::FlowBoxChild::new();
          // let box_ = gtk::Box::new(gtk4::Orientation::Horizontal, 1);
          // box_.append(&label);
          root.append(&label);
          child.set_visible(*is_visible);
          if *is_focused {
            root.select_child(&child)
          }
        }
        None => break,
      },
    }
    i += 1;
  }
  root.show();

  ()
}
