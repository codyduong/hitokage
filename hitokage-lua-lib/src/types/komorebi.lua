--- @meta hitokage.types.komorebi

--- @class KomorebiNotification
--- @field event KomorebiNotificationEvent
--- @field state KomorebiState

--- @class KomorebiState
--- @field cross_monitor_move_behaviour KomorebiMoveBehaviour
--- @field focus_follows_mouse KomorebiFocusFollowsMouseImplementation|nil
--- @field has_pending_raise_op boolean
--- @field is_paused boolean
--- @field monitors KomorebiMonitorRing
--- @field mouse_follows_focus boolean
--- @field new_window_behaviour KomorebiWindowContainerBehaviour
--- @field resize_delta integer
--- @field unmanaged_window_operation_behaviour KomorebiOperationBehaviour
--- @field work_area_offset KomorebiRect|nil

--- @class KomorebiMonitorRing
--- @field elements KomorebiMonitor[]
--- @field focused integer

--- @class KomorebiMonitor
--- @field device string
--- @field device_id string
--- @field id integer
--- @field last_focused_workspace integer|nil
--- @field name string
--- @field size KomorebiRect
--- @field window_based_work_area_offset KomorebiRect|nil
--- @field window_based_work_area_offset_limit integer
--- @field work_area_size KomorebiRect
--- @field workspace_names table<string, string>
--- @field workspaces KomorebiWorkspaceRing

--- @class KomorebiWorkspaceRing
--- @field elements KomorebiWorkspace[]
--- @field focused integer

--- @class KomorebiWorkspace
--- @field apply_window_based_work_area_offset boolean
--- @field container_padding integer|nil
--- @field containers Ring_for_Container
--- @field floating_windows KomorebiWindow[]
--- @field latest_layout KomorebiRect[]
--- @field layout KomorebiLayout
--- @field layout_flip KomorebiAxis|nil
--- @field layout_rules {index: integer, layout: KomorebiLayout}
--- @field maximized_window KomorebiWindow|nil
--- @field maximized_window_restore_idx integer|nil
--- @field monocle_container KomorebiContainer|nil
--- @field monocle_container_restore_idx integer|nil
--- @field name string|nil
--- @field resize_dimensions (KomorebiRect|nil)[]
--- @field tile boolean
--- @field workspace_padding integer|nil

--- @class Ring_for_Container
--- @field elements KomorebiContainer[]
--- @field focused integer

--- @class KomorebiContainer
--- @field id string
--- @field windows KomorebiWindowRing

--- @class KomorebiWindowRing
--- @field elements KomorebiWindow[]
--- @field focused integer

--- @class KomorebiWindow
--- @field hwnd integer

--- @class KomorebiLayout
--- @field Default KomorebiDefaultLayout
--- @field Custom KomorebiCustomLayout

--- @class KomorebiColumn
--- @field column "Primary"|"Secondary"|"Tertiary"
--- @field configuration KomorebiColumnWidth|KomorebiColumnSplitWithCapacity|KomorebiColumnSplit|nil

--- @class KomorebiColumnWidth
--- @field WidthPercentage number

--- @class KomorebiRect
--- @field bottom integer
--- @field left integer
--- @field right integer
--- @field top integer

--- @class KomorebiSocketMessage
--- @field type string
--- @field content any

--- @alias KomorebiNotificationEvent KomorebiWindowManagerEvent | KomorebiSocketMessage
--- @alias KomorebiDefaultLayout "BSP" | "Columns" | "Rows" | "VerticalStack" | "HorizontalStack" | "UltrawideVerticalStack" | "Grid" | "RightMainVerticalStack"
--- @alias KomorebiCustomLayout KomorebiColumn[]
--- @alias KomorebiColumnSplit "Horizontal" | "Vertical"
--- @alias KomorebiColumnSplitWithCapacity { Horizontal: integer } | { Vertical: integer }
--- @alias KomorebiMoveBehaviour "Swap" | "Insert" | "NoOp"
--- @alias KomorebiFocusFollowsMouseImplementation "Komorebi" | "Windows"
--- @alias KomorebiWindowContainerBehaviour "Create" | "Append"
--- @alias KomorebiOperationBehaviour "Op" | "NoOp"
--- @alias KomorebiAxis "Horizontal" | "Vertical" | "HorizontalAndVertical"
--- @alias KomorebiAnimationStyle "Linear" | "EaseInSine" | "EaseOutSine" | "EaseInOutSine" | "EaseInQuad" | "EaseOutQuad" | "EaseInOutQuad" | "EaseInCubic" | "EaseInOutCubic" | "EaseInQuart" | "EaseOutQuart" | "EaseInOutQuart" | "EaseInQuint" | "EaseOutQuint" | "EaseInOutQuint" | "EaseInExpo" | "EaseOutExpo" | "EaseInOutExpo" | "EaseInCirc" | "EaseOutCirc" | "EaseInOutCirc" | "EaseInBack" | "EaseOutBack" | "EaseInOutBack" | "EaseInElastic" | "EaseOutElastic" | "EaseInOutElastic" | "EaseInBounce" | "EaseOutBounce" | "EaseInOutBounce"
--- @alias KomorebiApplicationIdentifier "Exe" | "Class" | "Title" | "Path"
--- @alias KomorebiBorderImplementation "Komorebi" | "Windows"
--- @alias KomorebiBorderStyle "System" | "Rounded" | "Square"
--- @alias KomorebiCycleDirection "Previous" | "Next"
--- @alias KomorebiSizing "Increase" | "Decrease"
--- @alias KomorebiStackbarLabel "Process" | "Title"
--- @alias KomorebiStackbarMode "Always" | "Never" | "OnStack"
--- @alias KomorebiWindowManagerEvent { type: string, content: { event: KomorebiWinEvent, window: KomorebiWindow }[] }
--- @alias KomorebiWinEvent "AiaEnd" | "AiaStart" | "ConsoleCaret" | "ConsoleEnd" | "ConsoleEndApplication" | "ConsoleLayout" | "ConsoleStartApplication" | "ConsoleUpdateRegion" | "ConsoleUpdateScroll" | "ConsoleUpdateSimple" | "ObjectAcceleratorChange" | "ObjectCloaked" | "ObjectContentScrolled" | "ObjectCreate" | "ObjectDefActionChange" | "ObjectDescriptionChange" | "ObjectDestroy" | "ObjectDragCancel" | "ObjectDragComplete" | "ObjectDragDropped" | "ObjectDragEnter" | "ObjectDragLeave" | "ObjectDragStart" | "ObjectEnd" | "ObjectFocus" | "ObjectHelpChange" | "ObjectHide" | "ObjectHostedObjectsInvalidated" | "ObjectImeChange" | "ObjectImeHide" | "ObjectImeShow" | "ObjectInvoked" | "ObjectLiveRegionChanged" | "ObjectLocationChange" | "ObjectNameChange" | "ObjectParentChange" | "ObjectReorder" | "ObjectSelection" | "ObjectSelectionAdd" | "ObjectSelectionRemove" | "ObjectSelectionWithin" | "ObjectShow" | "ObjectStateChange" | "ObjectTextEditConversionTargetChanged" | "ObjectTextSelectionChanged" | "ObjectUncloaked" | "ObjectValueChange" | "OemDefinedEnd" | "OemDefinedStart" | "SystemAlert" | "SystemArrangementPreview" | "SystemCaptureEnd" | "SystemCaptureStart" | "SystemContextHelpEnd" | "SystemContextHelpStart" | "SystemDesktopSwitch" | "SystemDialogEnd" | "SystemDialogStart" | "SystemDragDropEnd" | "SystemDragDropStart" | "SystemEnd" | "SystemForeground" | "SystemImeKeyNotification" | "SystemMenuEnd" | "SystemMenuPopupEnd" | "SystemMenuPopupStart" | "SystemMenuStart" | "SystemMinimizeEnd" | "SystemMinimizeStart" | "SystemMoveSizeEnd" | "SystemMoveSizeStart" | "SystemScrollingEnd" | "SystemScrollingStart" | "SystemSound" | "SystemSwitchEnd" | "SystemSwitchStart" | "SystemSwitcherAppDropped" | "SystemSwitcherAppGrabbed" | "SystemSwitcherAppOverTarget" | "SystemSwitcherCancelled" | "UiaEventIdSEnd" | "UiaEventIdStart" | "UiaPropIdSEnd" | "UiaPropIdStart"
