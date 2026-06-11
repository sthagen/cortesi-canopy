// Ruskel skeleton - syntactically valid Rust with implementation omitted.
// settings: target=crates/canopy, visibility=public, auto_impls=false, blanket_impls=false

pub mod canopy {
    //! Canopy: A terminal UI library.
    //!
    //! Canopy is a terminal UI library for building interactive applications.
    //! It provides an arena-based widget system with focus management, styling,
    //! and event handling.
    //!
    //! # Quick Start
    //!
    //! The main entry points are:
    //! - [`Canopy`] - The core application state
    //! - [`Widget`] - The trait implemented by all widgets
    //! - [`Context`] - The mutation API available to widgets
    //!
    //! # Module Organization
    //!
    //! - [`geom`] - Geometry primitives (Rect, Point, Size, etc.)

    pub mod layout {
        //! Layout types for configuring node positioning and sizing.

        /// Stack direction for children.
        #[derive(Clone, Copy, Debug, Default, StructuralPartialEq, PartialEq, Eq)]
        pub enum Direction {
            /// Stack children vertically (column).
            Column,
            /// Stack children horizontally (row).
            Row,
            /// Children overlap in the same space (painter's algorithm - last child on top).
            Stack,
        }

        impl Direction {
            /// Size along the main axis.
            pub fn main_size(&self, size: Size<u32>) -> u32 {}

            /// Size along the cross axis.
            pub fn cross_size(&self, size: Size<u32>) -> u32 {}

            /// Construct a size from main and cross axis values.
            pub fn size_from_main_cross(&self, main: u32, cross: u32) -> Size<u32> {}
        }

        /// Alignment along an axis.
        #[derive(Clone, Copy, Debug, Default, StructuralPartialEq, PartialEq, Eq)]
        pub enum Align {
            /// Align to the start of the axis.
            Start,
            /// Align to the center of the axis.
            Center,
            /// Align to the end of the axis.
            End,
        }

        /// Display mode for layout participation.
        #[derive(Clone, Copy, Debug, StructuralPartialEq, PartialEq, Eq)]
        pub enum Display {
            /// Node participates in layout and rendering.
            Block,
            /// Node is removed from layout and not rendered.
            None,
        }

        /// Sizing strategy for a single axis.
        #[derive(Clone, Copy, Debug, StructuralPartialEq, PartialEq, Eq)]
        pub enum Sizing {
            /// Size derives from `measure()` or wrapping children.
            Measure,
            /// Weighted share of remaining space along the axis.
            Flex(u32),
        }

        /// Invalid layout configuration.
        #[derive(Clone, Debug, StructuralPartialEq, PartialEq, Eq, Error, Display)]
        pub enum LayoutValidationError {
            /// A minimum bound exceeds the corresponding maximum bound.
            MinExceedsMax {
                /// Layout axis name.
                axis: &'static str,
                /// Minimum bound.
                min: u32,
                /// Maximum bound.
                max: u32,
            },
            /// A flex sizing strategy has a zero weight.
            ZeroFlexWeight {
                /// Layout axis name.
                axis: &'static str,
            },
            /// Padding on an axis overflows `u32`.
            PaddingOverflow {
                /// Padding axis name.
                axis: &'static str,
            },
        }

        /// Edge insets for padding.
        #[derive(Clone, Copy, Debug, Default, StructuralPartialEq, PartialEq, Eq)]
        pub struct Edges<T> {
            /// Top edge.
            pub top: T,
            /// Right edge.
            pub right: T,
            /// Bottom edge.
            pub bottom: T,
            /// Left edge.
            pub left: T,
        }

        impl<T: Copy> Edges<T> {
            /// Create edges with uniform length on all sides.
            pub fn all(v: T) -> Self {}

            /// Create edges with symmetric vertical and horizontal lengths.
            pub fn symmetric(vertical: T, horizontal: T) -> Self {}

            /// Create edges from individual values.
            pub fn new(top: T, right: T, bottom: T, left: T) -> Self {}
        }

        impl Edges<u32> {
            /// Total horizontal padding.
            pub fn horizontal(&self) -> u32 {}

            /// Total vertical padding.
            pub fn vertical(&self) -> u32 {}
        }

        /// Size with width and height.
        pub type Size<T = u32> = crate::geom::Size<T>;

        /// Layout configuration for a node.
        #[derive(Clone, Copy, Debug, StructuralPartialEq, PartialEq, Eq, Default)]
        pub struct Layout {
            /// Whether this node participates in layout/render.
            pub display: Display,
            /// Stack direction for children.
            pub direction: Direction,
            /// Width sizing strategy (outer size).
            pub width: Sizing,
            /// Height sizing strategy (outer size).
            pub height: Sizing,
            /// Minimum outer width constraint (cells).
            pub min_width: Option<u32>,
            /// Maximum outer width constraint (cells).
            pub max_width: Option<u32>,
            /// Minimum outer height constraint (cells).
            pub min_height: Option<u32>,
            /// Maximum outer height constraint (cells).
            pub max_height: Option<u32>,
            /// Allow horizontal overflow during measurement.
            pub overflow_x: bool,
            /// Allow vertical overflow during measurement.
            pub overflow_y: bool,
            /// Structural padding inside the widget (cells).
            pub padding: Edges<u32>,
            /// Gap between children along the main axis (cells).
            pub gap: u32,
            /// Horizontal alignment of children within content area.
            pub align_horizontal: Align,
            /// Vertical alignment of children within content area.
            pub align_vertical: Align,
        }

        impl Layout {
            /// Column layout with measured sizing on both axes.
            pub fn column() -> Self {}

            /// Row layout with measured sizing on both axes.
            pub fn row() -> Self {}

            /// Stack layout where children overlap in the same space.
            pub fn stack() -> Self {}

            /// Fill available space with flex sizing on both axes.
            pub fn fill() -> Self {}

            /// Remove this node from layout and rendering.
            pub fn none(self) -> Self {}

            /// Set width to flex with the provided weight (clamped to at least 1).
            pub fn flex_horizontal(self, weight: u32) -> Self {}

            /// Set height to flex with the provided weight (clamped to at least 1).
            pub fn flex_vertical(self, weight: u32) -> Self {}

            /// Set the minimum outer width.
            pub fn min_width(self, n: u32) -> Self {}

            /// Set the maximum outer width.
            pub fn max_width(self, n: u32) -> Self {}

            /// Set the minimum outer height.
            pub fn min_height(self, n: u32) -> Self {}

            /// Set the maximum outer height.
            pub fn max_height(self, n: u32) -> Self {}

            /// Allow horizontal overflow during measurement.
            pub fn overflow_x(self) -> Self {}

            /// Allow vertical overflow during measurement.
            pub fn overflow_y(self) -> Self {}

            /// Convenience: fixed outer width without a `Fixed` enum.
            pub fn fixed_width(self, n: u32) -> Self {}

            /// Convenience: fixed outer height without a `Fixed` enum.
            pub fn fixed_height(self, n: u32) -> Self {}

            /// Set padding edges.
            pub fn padding(self, edges: Edges<u32>) -> Self {}

            /// Set the main-axis gap between children.
            pub fn gap(self, n: u32) -> Self {}

            /// Set horizontal alignment of children within content area.
            pub fn align_horizontal(self, align: Align) -> Self {}

            /// Set vertical alignment of children within content area.
            pub fn align_vertical(self, align: Align) -> Self {}

            /// Center children both horizontally and vertically.
            pub fn align_center(self) -> Self {}

            /// Set the layout direction.
            pub fn direction(self, direction: Direction) -> Self {}

            /// Set width sizing strategy directly.
            pub fn width(self, sizing: Sizing) -> Self {}

            /// Set height sizing strategy directly.
            pub fn height(self, sizing: Sizing) -> Self {}

            /// Set both axes to Measure sizing.
            pub fn measured(self) -> Self {}

            /// Validate this layout configuration.
            pub fn validate(&self) -> Result<(), LayoutValidationError> {}
        }

        /// Content-box measurement constraints.
        #[derive(Clone, Copy, Debug, StructuralPartialEq, PartialEq, Eq, Hash)]
        pub enum Constraint {
            /// No constraint on this axis.
            Unbounded,
            /// The engine guarantees at most n cells on this axis.
            AtMost(u32),
            /// The engine guarantees exactly n cells on this axis.
            Exact(u32),
        }

        impl Constraint {
            /// Return true if this constraint is exact.
            pub fn is_exact(self) -> bool {}

            /// Return the maximum bound implied by the constraint.
            pub fn max_bound(self) -> u32 {}
        }

        /// Constraints for measuring a widget's content box.
        #[derive(Clone, Copy, Debug, StructuralPartialEq, PartialEq, Eq, Hash)]
        pub struct MeasureConstraints {
            /// Width constraint.
            pub width: Constraint,
            /// Height constraint.
            pub height: Constraint,
        }

        impl MeasureConstraints {
            /// Leaf widgets: clamp a content size to these constraints and return Fixed.
            pub fn clamp(&self, content: Size<u32>) -> Measurement {}

            /// Containers: request wrapping.
            pub fn wrap(&self) -> Measurement {}

            /// Clamp a size to these constraints.
            pub fn clamp_size(&self, content: Size<u32>) -> Size<u32> {}

            /// True if the main axis is exact.
            pub fn main_is_exact(&self, direction: Direction) -> bool {}

            /// True if the cross axis is exact.
            pub fn cross_is_exact(&self, direction: Direction) -> bool {}

            /// Return the main axis constraint.
            pub fn main(&self, direction: Direction) -> Constraint {}

            /// Return the cross axis constraint.
            pub fn cross(&self, direction: Direction) -> Constraint {}
        }

        /// Result of measuring a widget's content box.
        #[derive(Clone, Copy, Debug, StructuralPartialEq, PartialEq, Eq)]
        pub enum Measurement {
            /// Fixed content size for leaf widgets.
            Fixed(Size<u32>),
            /// Wrap children: engine computes content size from children.
            Wrap,
        }

        /// Canvas context for computing scrollable extents.
        pub struct CanvasContext<'a> {}

        impl<'a> CanvasContext<'a> {
            /// Construct a canvas context from a child slice.
            pub fn new(children: &'a [CanvasChild]) -> Self {}

            /// Child layout results in this node's content coordinate space.
            pub fn children(&self) -> &[CanvasChild] {}

            /// Extent of children outer rects.
            pub fn children_extent(&self) -> Size<u32> {}
        }

        /// Child layout results for canvas computations.
        #[derive(Clone, Copy, Debug, StructuralPartialEq, PartialEq, Eq)]
        pub struct CanvasChild {
            /// Child outer rect relative to this node's content origin.
            pub rect: crate::geom::Rect,
            /// Child canvas size in the child's content coordinates.
            pub canvas: Size<u32>,
        }

        impl CanvasChild {
            /// Construct a new canvas child.
            pub fn new(rect: Rect, canvas: Size<u32>) -> Self {}
        }

        /// Clamp a flex weight to at least 1.
        pub fn clamp_weight(weight: u32) -> u32 {}
    }

    pub mod prelude {
        //! Convenience re-exports for common Canopy types.

        /// Application runtime state and renderer coordination.
        pub struct Canopy {}

        impl Canopy {
            /// Construct a new Canopy instance.
            pub fn new() -> Self {}

            /// Return a handle for submitting automation work to this app's UI thread.
            pub fn automation_handle(&self) -> AutomationHandle {}

            /// Return the root node ID.
            pub fn root_id(&self) -> NodeId {}

            /// Create a detached widget node.
            pub fn create_detached<W>(&mut self, widget: W) -> TypedId<W>
            where
                W: Widget + 'static, {
            }

            /// Replace the root's children with a single node.
            pub fn set_root_child(&mut self, child: impl Into<NodeId>) -> Result<()> {}

            /// Return the active style map.
            pub fn style(&self) -> &StyleMap {}

            /// Mutate the active style map before the next render.
            pub fn style_mut(&mut self) -> &mut StyleMap {}

            /// Replace the active style map before the next render.
            pub fn set_style(&mut self, style: StyleMap) {}

            /// Register a backend controller.
            pub fn register_backend<T: BackendControl + 'static>(&mut self, be: T) {}

            /// Get a reference to the current render buffer, if any.
            pub fn buf(&self) -> Option<&TermBuf> {}

            /// Run a compiled script by id on the target node.
            pub fn run_script(
                &mut self,
                node_id: impl Into<NodeId>,
                sid: script::ScriptId,
            ) -> Result<()> {
            }

            /// Compile a script and return its identifier.
            pub fn compile_script(&mut self, source: &str) -> Result<script::ScriptId> {}

            /// Evaluate a Luau source string in the current app context.
            pub fn eval_script(&mut self, source: &str) -> Result<()> {}

            /// Evaluate a Luau source string and return its value.
            pub fn eval_script_value(&mut self, source: &str) -> Result<commands::ArgValue> {}

            /// Evaluate a Luau source string with a cooperative timeout.
            pub fn eval_script_value_with_timeout(
                &mut self,
                source: &str,
                timeout: Duration,
            ) -> Result<commands::ArgValue> {
            }

            /// Evaluate the app's built-in default bindings script.
            pub fn run_default_script(&mut self, source: &str) -> Result<()> {}

            /// Register a Luau script as the default bindings for a widget namespace.
            pub fn register_default_bindings(&mut self, name: &str, script: &str) -> Result<()> {}

            /// Register a named fixture available to headless and live automation.
            pub fn register_fixture(&mut self, fixture: Fixture) -> Result<()> {}

            /// Return registered fixture metadata in stable name order.
            pub fn fixture_infos(&self) -> Vec<FixtureInfo> {}

            /// Apply a named fixture to the current app instance.
            pub fn apply_fixture(&mut self, name: &str) -> Result<()> {}

            /// Run a closure against the root context.
            pub fn with_root_context<R>(
                &mut self,
                f: impl FnOnce(&mut dyn crate::Context) -> Result<R>,
            ) -> Result<R> {
            }

            /// Type-check a Luau source string against the finalized app API.
            pub fn check_script(&mut self, source: &str) -> Result<script::ScriptCheckResult> {}

            /// Drain and return log lines recorded by the most recent script evaluation.
            pub fn take_script_logs(&self) -> Vec<String> {}

            /// Drain and return assertion outcomes from the most recent script evaluation.
            pub fn take_script_assertions(&self) -> Vec<script::ScriptAssertion> {}

            /// Evaluate a Luau config file from disk.
            pub fn run_config(&mut self, path: &FsPath) -> Result<()> {}

            /// Remove a binding by ID. Returns true if a binding was removed.
            pub fn unbind(&mut self, id: inputmap::BindingId) -> bool {}

            /// Remove bindings for a key input, optionally filtered by mode and path.
            pub fn unbind_key_input<K>(
                &mut self,
                key: K,
                mode: Option<&str>,
                path_filter: Option<&str>,
            ) -> usize
            where
                key::Key: From<K>, {
            }

            /// Remove bindings for a mouse input, optionally filtered by mode and path.
            pub fn unbind_mouse_input<K>(
                &mut self,
                mouse: K,
                mode: Option<&str>,
                path_filter: Option<&str>,
            ) -> usize
            where
                mouse::Mouse: From<K>, {
            }

            /// Remove all bindings from all modes.
            pub fn clear_bindings(&mut self) -> usize {}

            /// Return all bindings defined for a mode.
            pub fn bindings_for_mode(&self, mode: &str) -> Vec<inputmap::BindingInfo<'_>> {}

            /// Return bindings in a mode that match a specific path.
            pub fn bindings_matching_path(
                &self,
                mode: &str,
                path: &Path,
            ) -> Vec<inputmap::MatchedBindingInfo<'_>> {
            }

            /// Return the active input mode.
            pub fn input_mode(&self) -> &str {}

            /// Set the active input mode.
            pub fn set_input_mode(&mut self, mode: &str) -> Result<()> {}

            /// Bind a key or mouse input to switch the active input mode.
            pub fn bind_input_mode(
                &mut self,
                mode: &str,
                input: inputmap::InputSpec,
                path_filter: &str,
                next_mode: &str,
            ) -> Result<inputmap::BindingId> {
            }

            /// Return the most recent key or mouse route trace.
            pub fn route_trace(&self) -> &[RouteTraceEntry] {}

            /// Load the commands from a command node using the default node name.
            /// Returns an error if any command id is already registered.
            pub fn add_commands<T: commands::CommandNode>(&mut self) -> Result<()> {}

            /// Finalize the script API surface for this app.
            pub fn finalize_api(&mut self) -> Result<()> {}

            /// Return the rendered Luau definition file for this app.
            pub fn script_api(&self) -> &str {}

            /// Output a formatted table of commands to a writer.
            ///
            /// If `include_hidden` is false, commands with `doc.hidden = true` are excluded.
            pub fn print_command_table(
                &self,
                w: &mut dyn Write,
                include_hidden: bool,
            ) -> Result<()> {
            }

            /// Return command availability from the current focus position.
            ///
            /// This computes which commands would resolve to a target if dispatched from the current
            /// focus. For each command:
            /// - Free commands always have `resolution = Some(Free)`
            /// - Node-routed commands have `resolution = Some(Subtree{..})` or `Some(Ancestor{..})`
            ///   if a matching node exists, `None` otherwise
            pub fn command_availability_from_focus(
                &self,
            ) -> Vec<commands::CommandAvailability<'_>> {
            }

            /// Return command availability from a specific node.
            ///
            /// Computes which commands would dispatch to a target, using the same resolution logic
            /// as `commands::dispatch`:
            /// 1. First search the subtree rooted at `start` in pre-order
            /// 2. Then walk ancestors
            pub fn command_availability_from_node(
                &self,
                start: NodeId,
            ) -> Vec<commands::CommandAvailability<'_>> {
            }

            /// Generate a contextual help snapshot for the current focus.
            ///
            /// The snapshot includes:
            /// - Bindings that would match from the focus path
            /// - Commands with their availability status
            pub fn help_snapshot(&self) -> super::help::HelpSnapshot<'_> {}

            /// Build a diagnostic dump with tree, focus, and binding details.
            pub fn diagnostic_dump(&self, target: NodeId) -> String {}

            /// Has the focus path status of this node changed since the last render sweep?
            pub fn node_focus_path_changed(&self, node_id: impl Into<NodeId>) -> bool {}

            /// Render the widget tree. All visible nodes are rendered.
            pub fn render<R: RenderBackend>(&mut self, be: &mut R) -> Result<()> {}

            /// Set the size on the root node.
            pub fn set_root_size(&mut self, size: Size) -> Result<()> {}
        }

        /// A typed key for keyed children.
        ///
        /// This trait associates a string key with a specific widget type, providing
        /// compile-time type safety for keyed child access.
        ///
        /// Use the [`key!`] macro to define keys:
        ///
        /// ```ignore
        /// // Key with same name as widget type
        /// Editor);
        ///
        /// // Key with custom name
        /// ModalSlot: Modal);
        /// ```
        pub trait ChildKey {
            type Widget: Widget + 'static;
            const KEY: &'static str;
        }

        pub use crate::CommandArg;
        /// Command dispatch context helpers.
        pub trait CommandContext: Context {
            /// Dispatch a prepared command invocation from the current node.
            fn dispatch_prepared_command(
                &mut self,
                command: &CommandInvocation,
            ) -> StdResult<ArgValue, CommandError> {
            }
        }

        pub use crate::CommandEnum;
        /// Mutable context available to widgets during event handling.
        pub trait Context: ReadContext {
            /// Focus a node. Returns `true` if focus changed.
            fn set_focus(&mut self, node: NodeId) -> bool;

            /// Move focus in a specified direction within the current node's subtree.
            fn focus_dir(&mut self, dir: Direction) {}

            /// Move focus in a specified direction within the specified subtree.
            fn focus_dir_in(&mut self, root: NodeId, dir: Direction);

            /// Move focus in a specified direction within the entire tree (from root).
            fn focus_dir_global(&mut self, dir: Direction) {}

            /// Focus the first node that accepts focus in the current node's subtree.
            fn focus_first(&mut self) {}

            /// Focus the first node that accepts focus in the specified subtree.
            fn focus_first_in(&mut self, root: NodeId);

            /// Focus the first node that accepts focus in the entire tree (from root).
            fn focus_first_global(&mut self) {}

            /// Focus the next node in the current node's subtree.
            fn focus_next(&mut self) {}

            /// Focus the next node in the specified subtree.
            fn focus_next_in(&mut self, root: NodeId);

            /// Focus the next node in the entire tree (from root).
            fn focus_next_global(&mut self) {}

            /// Focus the previous node in the current node's subtree.
            fn focus_prev(&mut self) {}

            /// Focus the previous node in the specified subtree.
            fn focus_prev_in(&mut self, root: NodeId);

            /// Focus the previous node in the entire tree (from root).
            fn focus_prev_global(&mut self) {}

            /// Move focus to the right within the current node's subtree.
            fn focus_right(&mut self) {}

            /// Move focus to the right within the specified subtree.
            fn focus_right_in(&mut self, root: NodeId) {}

            /// Move focus to the right within the entire tree (from root).
            fn focus_right_global(&mut self) {}

            /// Move focus to the left within the current node's subtree.
            fn focus_left(&mut self) {}

            /// Move focus to the left within the specified subtree.
            fn focus_left_in(&mut self, root: NodeId) {}

            /// Move focus to the left within the entire tree (from root).
            fn focus_left_global(&mut self) {}

            /// Move focus upward within the current node's subtree.
            fn focus_up(&mut self) {}

            /// Move focus upward within the specified subtree.
            fn focus_up_in(&mut self, root: NodeId) {}

            /// Move focus upward within the entire tree (from root).
            fn focus_up_global(&mut self) {}

            /// Move focus downward within the current node's subtree.
            fn focus_down(&mut self) {}

            /// Move focus downward within the specified subtree.
            fn focus_down_in(&mut self, root: NodeId) {}

            /// Move focus downward within the entire tree (from root).
            fn focus_down_global(&mut self) {}

            /// Capture mouse events for the current node. Returns `true` if capture changed.
            fn capture_mouse(&mut self) -> bool;

            /// Release mouse capture if held by the current node. Returns `true` if capture changed.
            fn release_mouse(&mut self) -> bool;

            /// Scroll the view to the specified position. Returns `true` if movement occurred.
            fn scroll_to(&mut self, x: u32, y: u32) -> bool;

            /// Scroll the view by the given offsets. Returns `true` if movement occurred.
            fn scroll_by(&mut self, x: i32, y: i32) -> bool;

            /// Scroll the view up by one page. Returns `true` if movement occurred.
            fn page_up(&mut self) -> bool {}

            /// Scroll the view down by one page. Returns `true` if movement occurred.
            fn page_down(&mut self) -> bool {}

            /// Scroll the view up by one line. Returns `true` if movement occurred.
            fn scroll_up(&mut self) -> bool {}

            /// Scroll the view down by one line. Returns `true` if movement occurred.
            fn scroll_down(&mut self) -> bool {}

            /// Scroll the view left by one line. Returns `true` if movement occurred.
            fn scroll_left(&mut self) -> bool {}

            /// Scroll the view right by one line. Returns `true` if movement occurred.
            fn scroll_right(&mut self) -> bool {}

            /// Mark this node dirty so the next frame re-runs layout.
            fn invalidate_layout(&mut self);

            /// Update the layout for the current node.
            fn with_layout(&mut self, f: &mut dyn FnMut(&mut Layout)) -> Result<()> {}

            /// Update the layout for a specific node.
            fn with_layout_of(
                &mut self,
                node: NodeId,
                f: &mut dyn FnMut(&mut Layout),
            ) -> Result<()>;

            /// Create a new widget node detached from the tree.
            fn create_detached_boxed(&mut self, widget: Box<dyn Widget>) -> NodeId;

            /// Execute a closure with mutable access to a widget and its node-bound context.
            fn with_widget_mut(
                &mut self,
                node: NodeId,
                f: &mut dyn FnMut(&mut dyn Widget, &mut dyn Context) -> Result<()>,
            ) -> Result<()>;

            /// Dispatch a command relative to this node.
            fn dispatch_command(
                &mut self,
                cmd: &CommandInvocation,
            ) -> StdResult<ArgValue, CommandError>;

            /// Dispatch a command with an explicit command-scope frame.
            fn dispatch_command_scoped(
                &mut self,
                frame: CommandScopeFrame,
                cmd: &CommandInvocation,
            ) -> StdResult<ArgValue, CommandError>;

            /// Return the current event snapshot for injection.
            fn current_event(&self) -> Option<&Event>;

            /// Return the current mouse event for injection.
            fn current_mouse_event(&self) -> Option<MouseEvent>;

            /// Return the current list-row context for injection.
            fn current_list_row(&self) -> Option<ListRowContext>;

            /// Add a boxed widget as a child of a specific parent and return the new node ID.
            fn add_child_to_boxed(
                &mut self,
                parent: NodeId,
                widget: Box<dyn Widget>,
            ) -> Result<NodeId>;

            /// Add a boxed widget as a keyed child of a specific parent and return the new node ID.
            fn add_child_to_keyed_boxed(
                &mut self,
                parent: NodeId,
                key: &str,
                widget: Box<dyn Widget>,
            ) -> Result<NodeId>;

            /// Attach a detached child to a parent.
            fn attach(&mut self, parent: NodeId, child: NodeId) -> Result<()>;

            /// Attach a detached child to a parent using a unique key.
            fn attach_keyed(&mut self, parent: NodeId, key: &str, child: NodeId) -> Result<()>;

            /// Detach a child from its parent.
            fn detach(&mut self, child: NodeId) -> Result<()>;

            /// Remove a node and all descendants from the arena.
            fn remove_subtree(&mut self, node: NodeId) -> Result<()>;

            /// Replace the children list for the current node.
            fn set_children(&mut self, children: Vec<NodeId>) -> Result<()> {}

            /// Replace the children list for a specific parent node.
            fn set_children_of(&mut self, parent: NodeId, children: Vec<NodeId>) -> Result<()>;

            /// Set the current node's visibility. Returns `true` if visibility changed.
            fn set_hidden(&mut self, hidden: bool) -> bool {}

            /// Set a specific node's visibility. Returns `true` if visibility changed.
            fn set_hidden_of(&mut self, node: NodeId, hidden: bool) -> bool;

            /// Hide the current node. Returns `true` if visibility changed.
            fn hide(&mut self) -> bool {}

            /// Hide a specific node. Returns `true` if visibility changed.
            fn hide_node(&mut self, node: NodeId) -> bool {}

            /// Show the current node. Returns `true` if visibility changed.
            fn show(&mut self) -> bool {}

            /// Show a specific node. Returns `true` if visibility changed.
            fn show_node(&mut self, node: NodeId) -> bool {}

            /// Start the backend renderer.
            fn start(&mut self) -> Result<()>;

            /// Stop the backend renderer, releasing control of the terminal.
            fn stop(&mut self) -> Result<()>;

            /// Request a cooperative shutdown with the provided status code.
            fn exit(&mut self, code: i32);

            /// Add an effect to a node that will be applied during rendering.
            /// Effects stack and inherit through the tree.
            fn push_effect(&mut self, node: NodeId, effect: Effect) -> Result<()>;

            /// Clear all effects on a node.
            fn clear_effects(&mut self, node: NodeId) -> Result<()>;

            /// Set whether a node should clear inherited effects before applying local ones.
            fn set_clear_inherited_effects(&mut self, node: NodeId, clear: bool) -> Result<()>;

            /// Set the style map to be used for rendering.
            /// The style change will be applied before the next render.
            fn set_style(&mut self, style: StyleMap);

            /// Request a help snapshot to be injected into the specified target node.
            ///
            /// This should be called before changing focus or layout, so the snapshot
            /// captures the pre-help context. After the current command returns, Canopy
            /// will capture the snapshot and inject it into the target widget.
            fn request_help_snapshot(&mut self, target: NodeId);

            /// Take the pending help snapshot, if any.
            ///
            /// This is called by help widgets to retrieve the snapshot that was
            /// captured when `request_help_snapshot` was called. Returns `None` if
            /// no snapshot is pending.
            fn take_help_snapshot(&mut self) -> Option<OwnedHelpSnapshot>;

            /// Request a diagnostic dump for a target node.
            fn request_diagnostic_dump(&mut self, target: NodeId);
        }

        /// The result of an event handler.
        #[derive(Debug, StructuralPartialEq, PartialEq, Eq, Clone)]
        pub enum EventOutcome {
            /// The event was processed and propagation stops.
            Handle,
            /// The event was processed without a state change and propagation stops.
            Consume,
            /// The event was not handled and will bubble up the tree.
            Ignore,
        }

        /// Focus-related context helpers.
        pub trait FocusContext: Context {
            /// Focus a specific node.
            fn focus_node(&mut self, node: NodeId) -> bool {}

            /// Move focus from the current node in a direction.
            fn move_focus(&mut self, direction: Direction) {}
        }

        /// Layout mutation context helpers.
        pub trait LayoutContext: Context {
            /// Replace a node's layout.
            fn replace_layout(&mut self, node: impl Into<NodeId>, layout: Layout) -> Result<()> {}
        }

        /// Validate a child view position against the parent canvas bounds.
        /// A trait that allows widgets to perform recursive initialization of themselves and their
        /// children.
        pub trait Loader {
            /// Load commands or resources into the canopy instance.
            /// Returns an error if loading fails.
            fn load(_: &mut Canopy) -> Result<()> {}
        }

        /// Opaque identifier for a node stored in the Core arena.
        #[derive(
            Copy, Clone, Default, Eq, StructuralPartialEq, PartialEq, Ord, PartialOrd, Hash, Debug,
        )]
        pub struct NodeId(_);

        impl From<KeyData> for NodeId {
            fn from(k: KeyData) -> Self {}
        }

        impl Key for NodeId {
            fn data(&self) -> KeyData {}
        }

        impl<T> From<TypedId<T>> for NodeId {
            fn from(value: TypedId<T>) -> Self {}
        }

        /// A path of node name components.
        #[derive(Debug, Clone, StructuralPartialEq, PartialEq, Eq, FromStr, Display)]
        pub struct Path {}

        impl Path {
            /// Construct an empty path.
            pub fn empty() -> Self {}

            /// Parse and validate a path from a slash-separated string.
            pub fn parse(path: &str) -> Result<Self> {}

            /// Pop an item off the end of the path, modifying it in place. Return None
            /// if the path is empty.
            pub fn pop(&mut self) -> Option<String> {}

            /// Construct a path from a slice of components.
            pub fn new<I>(v: I) -> Self
            where
                I: IntoIterator,
                I::Item: AsRef<str>, {
            }
        }

        impl From<Vec<String>> for Path {
            fn from(path: Vec<String>) -> Self {}
        }

        impl From<&[&str]> for Path {
            fn from(v: &[&str]) -> Self {}
        }

        impl From<&str> for Path {
            fn from(v: &str) -> Self {}
        }

        /// A validated path filter used to search node paths.
        ///
        /// Filters support `*` for one component and `**` for zero or more components.
        /// Literal components must be valid [`NodeName`] values.
        #[derive(Debug, Clone, FromStr)]
        pub struct PathFilter {}

        impl PathFilter {
            /// Compile a validated path filter.
            pub fn new(filter: &str) -> Result<Self> {}

            /// Compile a filter after normalizing it to a full-path match.
            pub fn normalized(filter: &str) -> Result<Self> {}

            /// Return the original filter string.
            pub fn as_str(&self) -> &str {}
        }

        /// Read-only context available to widgets during render and measure.
        pub trait ReadContext {
            /// The node currently being rendered.
            fn node_id(&self) -> NodeId;

            /// The root node of the tree.
            fn root_id(&self) -> NodeId;

            /// View information for the current node.
            fn view(&self) -> &View;

            /// Cached layout configuration for the current node.
            fn layout(&self) -> Layout;

            /// View information for a specific node.
            fn node_view(&self, node: NodeId) -> Option<View>;

            /// Widget type identifier for a specific node.
            fn node_type_id(&self, node: NodeId) -> Option<TypeId>;

            /// Canvas size for the current node.
            fn canvas(&self) -> Size {}

            /// Visible view rectangle in content coordinates.
            fn view_rect(&self) -> Rect {}

            /// Visible view rectangle in local outer coordinates.
            fn view_rect_local(&self) -> Rect {}

            /// Local outer rectangle for this node.
            fn outer_rect_local(&self) -> Rect {}

            /// Children of the current node in tree order.
            fn children(&self) -> Vec<NodeId> {}

            /// Children of a specific node in tree order.
            fn children_of(&self, node: NodeId) -> Vec<NodeId>;

            /// Does the current node have focus?
            fn is_focused(&self) -> bool;

            /// Does the specified node have focus?
            fn node_is_focused(&self, node: NodeId) -> bool;

            /// Is the current node on the focus path?
            fn is_on_focus_path(&self) -> bool;

            /// Is the specified node on the focus path?
            fn node_is_on_focus_path(&self, node: NodeId) -> bool;

            /// Return the focus path for the subtree under `root`.
            fn focus_path(&self, root: NodeId) -> Path;

            /// Return the focused leaf under the subtree rooted at `root`.
            fn focused_leaf(&self, root: NodeId) -> Option<NodeId>;

            /// Return focusable leaves in pre-order under the subtree rooted at `root`.
            fn focusable_leaves(&self, root: NodeId) -> Vec<NodeId>;

            /// Return the parent of a node, or `None` if it is the root or not found.
            fn parent_of(&self, node: NodeId) -> Option<NodeId>;

            /// Return the path for a node relative to a root.
            fn node_path(&self, root: NodeId, node: NodeId) -> Path;

            /// Return a keyed child relative to the current node.
            fn child_keyed(&self, key: &str) -> Option<NodeId>;

            /// Return a keyed child relative to a specific parent node.
            fn child_keyed_in(&self, parent: NodeId, key: &str) -> Option<NodeId>;

            /// Current focus generation counter.
            fn current_focus_gen(&self) -> u64 {}

            /// Find the first node whose path matches the filter, relative to the current node.
            ///
            /// The filter is normalized to match full paths.
            fn find_node(&self, path_filter: &str) -> Option<NodeId> {}

            /// Find the first node whose path matches the validated filter.
            fn find_node_matching(&self, path_filter: &PathFilter) -> Option<NodeId> {}

            /// Find all nodes whose paths match the filter, relative to the current node.
            ///
            /// The filter is normalized to match full paths.
            fn find_nodes(&self, path_filter: &str) -> Vec<NodeId> {}

            /// Find all nodes whose paths match the validated filter.
            fn find_nodes_matching(&self, path_filter: &PathFilter) -> Vec<NodeId> {}

            /// Peek at the pending help snapshot, if any.
            ///
            /// This is used by help widgets to check if a snapshot is available
            /// during render, without consuming it.
            fn pending_help_snapshot(&self) -> Option<&OwnedHelpSnapshot>;
        }

        /// Scroll context helpers.
        pub trait ScrollContext: Context {
            /// Scroll to a typed point.
            fn scroll_to_point(&mut self, point: Point) -> bool {}
        }

        /// Slot helper for keyed children that caches the resolved typed ID.
        #[derive(Debug, Default)]
        pub struct Slot<K: ChildKey> {}

        impl<K: ChildKey> Slot<K> {
            /// Construct an empty slot.
            pub fn new() -> Self {}

            /// Clear any cached typed ID.
            pub fn clear(&mut self) {}

            /// Get or create the keyed child under the current node.
            pub fn get_or_create(
                &mut self,
                ctx: &mut dyn Context,
                make: impl FnOnce() -> K::Widget,
            ) -> Result<TypedId<K::Widget>> {
            }

            /// Get or create the keyed child under a specific parent node.
            pub fn get_or_create_in(
                &mut self,
                ctx: &mut dyn Context,
                parent: impl Into<NodeId>,
                make: impl FnOnce() -> K::Widget,
            ) -> Result<TypedId<K::Widget>> {
            }

            /// Execute a closure with a keyed child under the current node.
            pub fn with<R>(
                &mut self,
                ctx: &mut dyn Context,
                f: impl FnOnce(&mut K::Widget, &mut dyn Context) -> Result<R>,
            ) -> Result<R> {
            }

            /// Execute a closure with a keyed child under a specific parent node.
            pub fn with_in<R>(
                &mut self,
                ctx: &mut dyn Context,
                parent: impl Into<NodeId>,
                f: impl FnOnce(&mut K::Widget, &mut dyn Context) -> Result<R>,
            ) -> Result<R> {
            }
        }

        /// Style context helpers.
        pub trait StyleContext: Context {
            /// Queue a style map for the next render pass.
            fn replace_style(&mut self, style: StyleMap) {}
        }

        /// Tree mutation context helpers.
        pub trait TreeContext: Context {
            /// Add a typed child to a parent node.
            fn add_child_widget<W: Widget + 'static>(
                &mut self,
                parent: impl Into<NodeId>,
                widget: W,
            ) -> Result<TypedId<W>> {
            }

            /// Remove a subtree rooted at `node`.
            fn remove_node_subtree(&mut self, node: impl Into<NodeId>) -> Result<()> {}
        }

        /// Type-safe wrapper around a node identifier tied to a widget type.
        #[derive(Debug, StructuralPartialEq, PartialEq, Eq, Hash, Clone, Copy)]
        pub struct TypedId<T> {}

        impl<T> TypedId<T> {
            /// Wrap an untyped node identifier.
            pub fn new(id: NodeId) -> Self {}
        }

        impl<T> From<TypedId<T>> for NodeId {
            fn from(value: TypedId<T>) -> Self {}
        }

        /// Widgets are the behavior attached to nodes in the Core arena.
        pub trait Widget: Any + Send {
            /// Layout configuration for this widget.
            fn layout(&self) -> Layout {}

            /// Measure intrinsic content size (content box, excludes Layout padding).
            fn measure(&self, c: MeasureConstraints) -> Measurement {}

            /// Canvas size in content coordinates (for scrolling).
            ///
            /// `view` is this node's content size (outer minus padding).
            fn canvas(&self, view: Size<u32>, _ctx: &CanvasContext<'_>) -> Size<u32> {}

            /// Render this widget's own content. Does not render children.
            fn render(&mut self, _frame: &mut Render<'_>, _ctx: &dyn ReadContext) -> Result<()> {}

            /// Handle events.
            fn on_event(&mut self, _event: &Event, _ctx: &mut dyn Context) -> Result<EventOutcome> {
            }

            /// Attempt to focus this widget.
            ///
            /// Widgets can use the provided context to query their tree state (e.g., whether they have
            /// children) when deciding whether to accept focus.
            fn accept_focus(&self, _ctx: &dyn ReadContext) -> bool {}

            /// Cursor specification for focused widgets.
            fn cursor(&self) -> Option<cursor::Cursor> {}

            /// Scheduled poll endpoint.
            fn poll(&mut self, _ctx: &mut dyn Context) -> Option<Duration> {}

            /// Called exactly once when the widget is first mounted in the tree, before the first render.
            ///
            /// The framework guarantees single invocation via an internal `mounted` flag on each node.
            /// There is no need to guard against multiple calls within this method.
            fn on_mount(&mut self, _ctx: &mut dyn Context) -> Result<()> {}

            /// Validation hook before a node is removed from the arena.
            ///
            /// This hook must be side-effect free or safely repeatable.
            fn pre_remove(&mut self, _ctx: &mut dyn Context) -> Result<()> {}

            /// Called exactly once immediately before the node is removed from the arena.
            fn on_unmount(&mut self, _ctx: &mut dyn Context) {}

            /// Name used for commands and paths.
            fn name(&self) -> NodeName {}
        }

        pub use crate::command;
        pub use crate::derive_commands;
        /// This enum represents all the event types that drive the application.
        #[derive(Debug, Clone)]
        pub enum Event {
            /// A keystroke
            Key(key::Key),
            /// A mouse action
            Mouse(mouse::MouseEvent),
            /// Terminal resize
            Resize(crate::geom::Size),
            /// A poll event
            Poll(Vec<crate::NodeId>),
            /// Terminal has gained focus
            FocusGained,
            /// Terminal has lost focus
            FocusLost,
            /// Cut and paste
            Paste(String),
            /// Internal wake event used to service queued automation work.
            Wake,
        }

        impl Inject for crate::event::Event {
            fn inject(ctx: &dyn Context) -> Result<Self, InjectError> {}
        }

        /// A keystroke along with modifiers.
        /// A keystroke along with modifiers.
        #[derive(
            Debug,
            StructuralPartialEq,
            PartialEq,
            Eq,
            Clone,
            Copy,
            Hash,
            PartialEq,
            PartialEq,
            PartialEq,
            Display,
        )]
        pub struct Key {
            /// Modifier state.
            pub mods: Mods,
            /// Key code.
            pub key: KeyCode,
        }

        impl Key {
            /// Normalize key inputs for binding and matching.
            ///
            /// Normalization handles two common sources of divergence across terminals:
            ///
            /// - **Ctrl-modified ASCII control codes** (0x00–0x1F and 0x7F) are mapped to
            ///   canonical printable equivalents (e.g. 0x01 → `A`, 0x1B → `[`, 0x7F → `?`).
            ///   Some terminals emit control codes without setting the Ctrl modifier, so
            ///   these codes are treated as Ctrl-combinations even if Ctrl isn't reported.
            ///   We also map Ctrl+`_`, Ctrl+`?`, and Ctrl+`7` to `/` to align with common
            ///   `Ctrl+/` help bindings across keyboard layouts and terminal encodings.
            /// - **Shift handling** is applied after Ctrl canonicalization.
            ///
            /// Handling of the shift key is the most intricate part of this module.
            /// When we receive an event, it includes the shift modifier and also the
            /// modified character - e.g. "shift + A" or "shift + (". However, when
            /// users bind keys, it's more intuitive to bind just "A" or "(". We don't
            /// know what the keyboard mapping or input method is for the user - so it's
            /// not possible in a general way for us to map between, say, an input like
            /// "shift + 0" to the shifted key "(". Conversely, if we see an input of
            /// "shift + (", we don't know if the user pressed "shift + 0" or if they
            /// have a weird keyboard layout that actually permits "shift + (" without a
            /// shift conversion.
            ///
            /// To handle this, we have to make a lossy compromise. We define a
            /// normalisation applied to input for the purpose of key binding matching
            /// as follows:
            ///
            /// - If shift is present:
            ///     - If the key is ascii lowercase, convert it to uppercase and remove
            ///       shift
            ///     - If the key is one of a special class of characters that commonly
            ///       don't have a shift conversion (space, enter), leave shift intact
            ///     - in all other cases, just remove shift
            ///
            /// | input             | normalization    |
            /// |-------------------|------------------|
            /// | shift + A         | A                |
            /// | shift + a         | A                |
            /// | shift + )         | )                |
            /// | shift + enter     | shift + enter    |
            /// | shift + ctrl + A  | ctrl + A         |
            ///
            /// `normalize` must be called explicitly when needed - all comparison and
            /// conversion methods are literal and stright-forward, and don't perform
            /// normalization automatically.
            pub fn normalize(&self) -> Self {}

            /// Parse a key specification such as `ctrl-s`, `PageDown`, or `A`.
            pub fn parse_spec(spec: &str) -> Result<Self, String> {}
        }

        impl From<char> for Key {
            fn from(c: char) -> Self {}
        }

        impl From<KeyCode> for Key {
            fn from(c: KeyCode) -> Self {}
        }

        pub mod mouse {
            //! Mouse event types.

            /// An abstract specification for a mouse action.
            #[derive(
                Debug,
                Clone,
                Copy,
                Hash,
                StructuralPartialEq,
                PartialEq,
                Eq,
                PartialEq,
                PartialEq,
                PartialEq,
                PartialEq,
            )]
            pub struct Mouse {
                /// Mouse action type.
                pub action: Action,
                /// Mouse button.
                pub button: Button,
                /// Keyboard modifiers.
                pub modifiers: key::Mods,
            }

            impl Mouse {
                /// Parse a mouse specification such as `ScrollUp` or `ctrl-LeftDown`.
                pub fn parse_spec(spec: &str) -> Result<Self, String> {}
            }

            impl From<MouseEvent> for Mouse {
                fn from(o: MouseEvent) -> Self {}
            }

            impl From<Button> for Mouse {
                fn from(e: Button) -> Self {}
            }

            impl From<Action> for Mouse {
                fn from(e: Action) -> Self {}
            }

            impl Add<Button> for Mouse {
                type Output = Mouse;
                fn add(self, other: Button) -> Self::Output {}
            }

            impl Add<Action> for Mouse {
                type Output = Mouse;
                fn add(self, other: Action) -> Self::Output {}
            }

            impl Add<Mods> for Mouse {
                type Output = Mouse;
                fn add(self, other: key::Mods) -> Self::Output {}
            }

            /// Mouse button codes.
            #[derive(
                Debug, PartialOrd, StructuralPartialEq, PartialEq, Eq, Clone, Copy, Hash, PartialEq,
            )]
            pub enum Button {
                /// Left mouse button.
                Left,
                /// Right mouse button.
                Right,
                /// Middle mouse button.
                Middle,
                /// No button (for move/scroll).
                None,
            }

            /// Synthesize a Mouse specification - the action is assumed to be
            /// `Action::Down`.
            impl Add<Mods> for Button {
                type Output = Mouse;
                fn add(self, other: key::Mods) -> Self::Output {}
            }

            impl Add<Button> for key::Mods {
                type Output = Mouse;
                fn add(self, other: Button) -> Self::Output {}
            }

            impl Add<Action> for Button {
                type Output = Mouse;
                fn add(self, other: Action) -> Self::Output {}
            }

            impl Add<Button> for Action {
                type Output = Mouse;
                fn add(self, other: Button) -> Self::Output {}
            }

            impl From<Button> for Mouse {
                fn from(e: Button) -> Self {}
            }

            impl Add<Button> for Mouse {
                type Output = Mouse;
                fn add(self, other: Button) -> Self::Output {}
            }

            /// Mouse action kinds.
            #[derive(
                Debug,
                PartialOrd,
                StructuralPartialEq,
                PartialEq,
                Eq,
                Clone,
                Copy,
                Hash,
                PartialEq,
                PartialEq,
            )]
            pub enum Action {
                /// Button press.
                Down,
                /// Button release.
                Up,
                /// Mouse drag with button held.
                Drag,
                /// Mouse moved without button.
                Moved,
                /// Scroll wheel down.
                ScrollDown,
                /// Scroll wheel up.
                ScrollUp,
                /// Horizontal scroll left.
                ScrollLeft,
                /// Horizontal scroll right.
                ScrollRight,
            }

            impl Action {
                /// Is this a button-driven action?
                pub fn is_button(&self) -> bool {}
            }

            impl Add<Action> for Button {
                type Output = Mouse;
                fn add(self, other: Action) -> Self::Output {}
            }

            /// Synthesize a `Mouse` input specification by adding modifiers to an action.
            /// Assume that the button is `Button::None`.
            impl Add<Mods> for Action {
                type Output = Mouse;
                fn add(self, other: key::Mods) -> Self::Output {}
            }

            impl Add<Action> for key::Mods {
                type Output = Mouse;
                fn add(self, other: Action) -> Self::Output {}
            }

            impl Add<Button> for Action {
                type Output = Mouse;
                fn add(self, other: Button) -> Self::Output {}
            }

            impl From<Action> for Mouse {
                fn from(e: Action) -> Self {}
            }

            impl Add<Action> for Mouse {
                type Output = Mouse;
                fn add(self, other: Action) -> Self::Output {}
            }

            /// A mouse input event. This has the same fields as the `Mouse` event
            /// specification, but also includes a location.
            #[derive(Debug, Clone, Copy, PartialEq, PartialEq, PartialEq)]
            pub struct MouseEvent {
                /// Mouse action type.
                pub action: Action,
                /// Mouse button.
                pub button: Button,
                /// Keyboard modifiers.
                pub modifiers: key::Mods,
                /// Cursor location in local coordinates relative to the node view. To map
                /// back to screen coordinates, add the node view's outer top-left.
                pub location: crate::geom::Point,
            }

            impl Inject for crate::event::mouse::MouseEvent {
                fn inject(ctx: &dyn Context) -> Result<Self, InjectError> {}
            }

            impl From<MouseEvent> for Mouse {
                fn from(o: MouseEvent) -> Self {}
            }
        }

        pub use crate::geom::Point;
        pub use crate::geom::Rect;
        pub use crate::geom::Size;
        /// Define a typed key for keyed children.
        ///
        /// # Examples
        ///
        /// ```ignore
        /// // Simple form: key name matches widget type, string key is snake_case
        /// Editor);  // KEY = "Editor", Widget = Editor (private)
        /// pub Editor);  // same, but public
        ///
        /// // Custom name form: specify the widget type explicitly
        /// ModalSlot: Modal);  // KEY = "ModalSlot", Widget = Modal (private)
        /// pub ModalSlot: Modal);  // same, but public
        /// ```
        #[macro_export]
        macro_rules! key {
    ($vis:vis $name:ident) => { ... };
    ($vis:vis $name:ident : $widget:ty) => { ... };
}
        /// Alignment along an axis.
        #[derive(Clone, Copy, Debug, Default, StructuralPartialEq, PartialEq, Eq)]
        pub enum Align {
            /// Align to the start of the axis.
            Start,
            /// Align to the center of the axis.
            Center,
            /// Align to the end of the axis.
            End,
        }

        /// Content-box measurement constraints.
        #[derive(Clone, Copy, Debug, StructuralPartialEq, PartialEq, Eq, Hash)]
        pub enum Constraint {
            /// No constraint on this axis.
            Unbounded,
            /// The engine guarantees at most n cells on this axis.
            AtMost(u32),
            /// The engine guarantees exactly n cells on this axis.
            Exact(u32),
        }

        impl Constraint {
            /// Return true if this constraint is exact.
            pub fn is_exact(self) -> bool {}

            /// Return the maximum bound implied by the constraint.
            pub fn max_bound(self) -> u32 {}
        }

        /// Stack direction for children.
        #[derive(Clone, Copy, Debug, Default, StructuralPartialEq, PartialEq, Eq)]
        pub enum Direction {
            /// Stack children vertically (column).
            Column,
            /// Stack children horizontally (row).
            Row,
            /// Children overlap in the same space (painter's algorithm - last child on top).
            Stack,
        }

        impl Direction {
            /// Size along the main axis.
            pub fn main_size(&self, size: Size<u32>) -> u32 {}

            /// Size along the cross axis.
            pub fn cross_size(&self, size: Size<u32>) -> u32 {}

            /// Construct a size from main and cross axis values.
            pub fn size_from_main_cross(&self, main: u32, cross: u32) -> Size<u32> {}
        }

        /// Display mode for layout participation.
        #[derive(Clone, Copy, Debug, StructuralPartialEq, PartialEq, Eq)]
        pub enum Display {
            /// Node participates in layout and rendering.
            Block,
            /// Node is removed from layout and not rendered.
            None,
        }

        /// Layout configuration for a node.
        #[derive(Clone, Copy, Debug, StructuralPartialEq, PartialEq, Eq, Default)]
        pub struct Layout {
            /// Whether this node participates in layout/render.
            pub display: Display,
            /// Stack direction for children.
            pub direction: Direction,
            /// Width sizing strategy (outer size).
            pub width: Sizing,
            /// Height sizing strategy (outer size).
            pub height: Sizing,
            /// Minimum outer width constraint (cells).
            pub min_width: Option<u32>,
            /// Maximum outer width constraint (cells).
            pub max_width: Option<u32>,
            /// Minimum outer height constraint (cells).
            pub min_height: Option<u32>,
            /// Maximum outer height constraint (cells).
            pub max_height: Option<u32>,
            /// Allow horizontal overflow during measurement.
            pub overflow_x: bool,
            /// Allow vertical overflow during measurement.
            pub overflow_y: bool,
            /// Structural padding inside the widget (cells).
            pub padding: Edges<u32>,
            /// Gap between children along the main axis (cells).
            pub gap: u32,
            /// Horizontal alignment of children within content area.
            pub align_horizontal: Align,
            /// Vertical alignment of children within content area.
            pub align_vertical: Align,
        }

        impl Layout {
            /// Column layout with measured sizing on both axes.
            pub fn column() -> Self {}

            /// Row layout with measured sizing on both axes.
            pub fn row() -> Self {}

            /// Stack layout where children overlap in the same space.
            pub fn stack() -> Self {}

            /// Fill available space with flex sizing on both axes.
            pub fn fill() -> Self {}

            /// Remove this node from layout and rendering.
            pub fn none(self) -> Self {}

            /// Set width to flex with the provided weight (clamped to at least 1).
            pub fn flex_horizontal(self, weight: u32) -> Self {}

            /// Set height to flex with the provided weight (clamped to at least 1).
            pub fn flex_vertical(self, weight: u32) -> Self {}

            /// Set the minimum outer width.
            pub fn min_width(self, n: u32) -> Self {}

            /// Set the maximum outer width.
            pub fn max_width(self, n: u32) -> Self {}

            /// Set the minimum outer height.
            pub fn min_height(self, n: u32) -> Self {}

            /// Set the maximum outer height.
            pub fn max_height(self, n: u32) -> Self {}

            /// Allow horizontal overflow during measurement.
            pub fn overflow_x(self) -> Self {}

            /// Allow vertical overflow during measurement.
            pub fn overflow_y(self) -> Self {}

            /// Convenience: fixed outer width without a `Fixed` enum.
            pub fn fixed_width(self, n: u32) -> Self {}

            /// Convenience: fixed outer height without a `Fixed` enum.
            pub fn fixed_height(self, n: u32) -> Self {}

            /// Set padding edges.
            pub fn padding(self, edges: Edges<u32>) -> Self {}

            /// Set the main-axis gap between children.
            pub fn gap(self, n: u32) -> Self {}

            /// Set horizontal alignment of children within content area.
            pub fn align_horizontal(self, align: Align) -> Self {}

            /// Set vertical alignment of children within content area.
            pub fn align_vertical(self, align: Align) -> Self {}

            /// Center children both horizontally and vertically.
            pub fn align_center(self) -> Self {}

            /// Set the layout direction.
            pub fn direction(self, direction: Direction) -> Self {}

            /// Set width sizing strategy directly.
            pub fn width(self, sizing: Sizing) -> Self {}

            /// Set height sizing strategy directly.
            pub fn height(self, sizing: Sizing) -> Self {}

            /// Set both axes to Measure sizing.
            pub fn measured(self) -> Self {}

            /// Validate this layout configuration.
            pub fn validate(&self) -> Result<(), LayoutValidationError> {}
        }

        /// Constraints for measuring a widget's content box.
        #[derive(Clone, Copy, Debug, StructuralPartialEq, PartialEq, Eq, Hash)]
        pub struct MeasureConstraints {
            /// Width constraint.
            pub width: Constraint,
            /// Height constraint.
            pub height: Constraint,
        }

        impl MeasureConstraints {
            /// Leaf widgets: clamp a content size to these constraints and return Fixed.
            pub fn clamp(&self, content: Size<u32>) -> Measurement {}

            /// Containers: request wrapping.
            pub fn wrap(&self) -> Measurement {}

            /// Clamp a size to these constraints.
            pub fn clamp_size(&self, content: Size<u32>) -> Size<u32> {}

            /// True if the main axis is exact.
            pub fn main_is_exact(&self, direction: Direction) -> bool {}

            /// True if the cross axis is exact.
            pub fn cross_is_exact(&self, direction: Direction) -> bool {}

            /// Return the main axis constraint.
            pub fn main(&self, direction: Direction) -> Constraint {}

            /// Return the cross axis constraint.
            pub fn cross(&self, direction: Direction) -> Constraint {}
        }

        /// Result of measuring a widget's content box.
        #[derive(Clone, Copy, Debug, StructuralPartialEq, PartialEq, Eq)]
        pub enum Measurement {
            /// Fixed content size for leaf widgets.
            Fixed(Size<u32>),
            /// Wrap children: engine computes content size from children.
            Wrap,
        }

        /// Sizing strategy for a single axis.
        #[derive(Clone, Copy, Debug, StructuralPartialEq, PartialEq, Eq)]
        pub enum Sizing {
            /// Size derives from `measure()` or wrapping children.
            Measure,
            /// Weighted share of remaining space along the axis.
            Flex(u32),
        }

        /// A renderer that only renders to a specific rectangle within the target terminal buffer.
        pub struct Render<'a> {}

        impl<'a> Render<'a> {
            /// Construct a renderer for the given rectangle.
            pub fn new(
                stylemap: &'a StyleMap,
                style: &'a mut StyleManager,
                rect: geom::Rect,
            ) -> Self {
            }

            /// Set the effect stack for this renderer.
            pub fn with_effects(self, effects: &'a [Effect]) -> Self {}

            /// Apply the current effect stack to a style.
            /// Use this when you have a Style from a source other than the style manager.
            pub fn apply_effects(&self, style: Style) -> Style {}

            /// Resolve a style by name without applying effects.
            pub fn resolve_style_name_raw(&self, name: &str) -> Style {}

            /// Resolve a style by name and apply the current effect stack.
            pub fn resolve_style_name(&self, name: &str) -> Style {}

            /// Resolve a custom style at a point, applying the current effect stack.
            pub fn resolve_style_at(
                &self,
                style: Style,
                bounds: geom::Rect,
                point: geom::Point,
            ) -> ResolvedStyle {
            }

            /// Resolve a style by name at a point within bounds.
            pub fn resolve_style_name_at(
                &self,
                name: &str,
                bounds: geom::Rect,
                point: geom::Point,
            ) -> ResolvedStyle {
            }

            /// Push a style layer.
            pub fn push_layer(&mut self, name: &str) {}

            /// Fill a rectangle with a specified character. Writes out of bounds will be clipped.
            pub fn fill(&mut self, style: &str, r: geom::Rect, c: char) -> Result<()> {}

            /// Draw a solid frame
            pub fn solid_frame(&mut self, style: &str, f: geom::FrameRects, c: char) -> Result<()> {
            }

            /// Print text in the specified line. If the text is wider than the
            /// rectangle, it will be truncated; if it is shorter, it will be padded.
            pub fn text(&mut self, style: &str, l: geom::Line, txt: &str) -> Result<()> {}

            /// Write a single cell with a resolved style.
            pub fn put_cell(
                &mut self,
                style: ResolvedStyle,
                p: geom::Point,
                ch: char,
            ) -> Result<()> {
            }

            /// Write a grapheme with a resolved style, including continuation cells.
            pub fn put_grapheme(
                &mut self,
                style: ResolvedStyle,
                p: geom::Point,
                grapheme: &str,
            ) -> Result<()> {
            }

            /// Access the underlying buffer.
            pub fn buffer(&self) -> &TermBuf {}
        }

        /// A node name, which consists of lowercase ASCII alphanumeric characters, plus
        /// underscores.
        #[derive(
            Debug,
            Clone,
            StructuralPartialEq,
            PartialEq,
            Eq,
            Hash,
            FromStr,
            Display,
            PartialEq,
            PartialEq,
        )]
        pub struct NodeName {}

        impl NodeName {
            /// Create a new NodeName, returning an error if the string contains invalid
            /// characters.
            pub fn new(name: &str) -> Result<Self> {}

            /// Takes a string and munges it into a valid node name. It does this by
            /// first converting the string to snake case, then removing all invalid
            /// characters.
            pub fn convert(name: &str) -> Self {}
        }

        /// Converts a string into the standard node name format, and errors if it
        /// doesn't comply to the node name standard.
        impl TryFrom<&str> for NodeName {
            type Error = Error;
            fn try_from(name: &str) -> Result<Self> {}
        }

        /// A builder for creating reusable style specifications.
        ///
        /// Use this to define styles that can be applied to multiple paths.
        ///
        /// # Example
        ///
        /// ```ignore
        /// let selected = StyleBuilder::new()
        ///     .fg(solarized::BASE3)
        ///     .bg(solarized::BLUE)
        ///     .attrs(selected_attrs);
        ///
        /// style_map.rules()
        ///     .rule("item/selected").style(selected)
        ///     .apply();
        /// ```
        #[derive(Clone, Default, Debug, StructuralPartialEq, PartialEq)]
        pub struct StyleBuilder {}

        impl StyleBuilder {
            /// Create a new empty style builder.
            pub fn new() -> Self {}

            /// Set the foreground paint.
            pub fn fg(self, paint: impl Into<Paint>) -> Self {}

            /// Set the background paint.
            pub fn bg(self, paint: impl Into<Paint>) -> Self {}

            /// Add a single attribute.
            pub fn attr(self, attr: Attr) -> Self {}

            /// Set all attributes.
            pub fn attrs(self, attrs: AttrSet) -> Self {}
        }

        impl From<StyleBuilder> for PartialStyle {
            fn from(s: StyleBuilder) -> Self {}
        }

        /// Map of style paths to partial styles.
        #[derive(Debug, Default)]
        pub struct StyleMap {}

        impl StyleMap {
            /// Construct a style map with defaults.
            pub fn new() -> Self {}

            /// Begin a fluent rule-building chain.
            ///
            /// # Example
            ///
            /// ```ignore
            /// style_map.rules()
            ///     .fg("red/text", solarized::RED)
            ///     .fg("blue/text", solarized::BLUE)
            ///     .apply();
            /// ```
            pub fn rules(&mut self) -> StyleRules<'_> {}

            /// Insert a style attribute at a specified path.
            pub fn add_attr(&mut self, path: &str, attr: Attr) {}
        }

        /// Common result alias for Canopy operations.
        pub type Result<T> = error::Result<T>;
    }

    pub use canopy_geom as geom;
    /// A 2D terminal buffer of styled cells.
    #[derive(Clone, Debug)]
    pub struct TermBuf {}

    impl TermBuf {
        /// Construct a buffer filled with the given character and style.
        pub fn new(size: impl Into<Size>, ch: char, style: ResolvedStyle) -> Self {}

        /// Create an empty TermBuf filled with NULL characters.
        pub fn empty_with_style(size: impl Into<Size>, style: ResolvedStyle) -> Self {}

        /// Create an empty TermBuf filled with NULL characters.
        pub fn empty(size: impl Into<Size>) -> Self {}

        /// Copy non-empty cells from a rectangle of another TermBuf into this one
        pub fn copy(&mut self, src: &Self, rect: Rect) {}

        /// Copy non-empty cells from a source TermBuf into a destination rectangle
        pub fn copy_to_rect(&mut self, src: &Self, dest_rect: Rect) {}

        /// Return the buffer size.
        pub fn size(&self) -> Size {}

        /// Return the buffer bounds as a rectangle.
        pub fn rect(&self) -> Rect {}

        /// Fill a rectangle with a glyph and style.
        pub fn fill(&mut self, style: &ResolvedStyle, r: Rect, ch: char) {}

        /// Fill all empty cells with the given character and style.
        pub fn fill_empty(&mut self, ch: char, style: &ResolvedStyle) {}

        /// Overlay a cursor on a cell by adjusting its style.
        pub fn overlay_cursor(&mut self, location: Point, shape: cursor::CursorShape) {}

        /// Fill the frame outline with a glyph and style.
        pub fn solid_frame(&mut self, style: &ResolvedStyle, f: FrameRects, ch: char) {}

        /// Draw text clipped to the given line.
        pub fn text(&mut self, style: &ResolvedStyle, l: Line, txt: &str) {}

        /// Get a cell by position.
        pub fn get(&self, p: Point) -> Option<&Cell> {}

        /// Return the rendered screen as rows of cell strings.
        pub fn rows(&self) -> Vec<Vec<String>> {}

        /// Return the rendered screen as newline-joined plain text.
        pub fn screen_text(&self) -> String {}

        /// Diff this terminal buffer against a previous state, emitting changes
        /// to the provided render backend.
        pub fn diff<R: RenderBackend>(&self, prev: &Self, backend: &mut R) -> Result<()> {}

        /// Render this terminal buffer in full using the provided backend,
        /// batching runs of text with the same style.
        pub fn render<R: RenderBackend>(&self, backend: &mut R) -> Result<()> {}
    }

    pub mod testing {
        //! Testing utilities.

        pub mod backend {
            //! Backend utilities for tests.

            /// A handle to a vector that contains the result of the render.
            #[derive(Default)]
            pub struct TestBuf {
                /// Captured text fragments.
                pub text: Vec<String>,
            }

            impl TestBuf {
                /// Return true if no text has been captured.
                pub fn is_empty(&self) -> bool {}

                /// Return true if any captured line contains the provided substring.
                pub fn contains(&self, s: &str) -> bool {}
            }

            /// A render backend for testing, which logs render outcomes.
            pub struct TestRender {
                /// Shared buffer of captured text.
                pub text: std::sync::Arc<std::sync::Mutex<TestBuf>>,
            }

            impl TestRender {
                /// Create returns a `TestBuf` protected by a mutex, and a `TestRender`
                /// instance. The `TestBuf` can be used to access the result of the render
                /// for testing.
                pub fn create() -> (Arc<Mutex<TestBuf>>, Self) {}

                /// Render a node tree into the test buffer.
                pub fn render(&mut self, c: &mut Canopy) -> Result<()> {}

                /// Return the default style manager used in tests.
                pub fn styleman(&self) -> StyleManager {}

                /// Return captured text lines.
                pub fn buf_text(&self) -> Vec<String> {}

                /// Return true if no text has been captured.
                pub fn buf_empty(&self) -> bool {}

                /// Return true if any captured line contains the substring.
                pub fn contains_text(&self, txt: &str) -> bool {}
            }

            impl RenderBackend for TestRender {
                fn reset(&mut self) -> Result<()> {}

                fn flush(&mut self) -> Result<()> {}

                fn style(&mut self, _s: &ResolvedStyle) -> Result<()> {}

                fn text(&mut self, _loc: Point, txt: &str) -> Result<()> {}

                fn supports_char_shift(&self) -> bool {}

                fn shift_chars(&mut self, _loc: Point, _count: i32) -> Result<()> {}
            }

            /// A simple in-memory canvas for verifying render placement in tests.
            #[derive(Default)]
            pub struct CanvasBuf {
                /// Character cells.
                pub cells: Vec<Vec<char>>,
                /// Track which cells have been written to during a render.
                pub painted: Vec<Vec<bool>>,
            }

            /// A render backend that draws into an in-memory canvas.
            pub struct CanvasRender {
                /// Shared canvas buffer for render output.
                pub canvas: std::sync::Arc<std::sync::Mutex<CanvasBuf>>,
            }

            impl CanvasRender {
                /// Create a new canvas render backend.
                pub fn create(size: Size) -> (Arc<Mutex<CanvasBuf>>, Self) {}
            }

            impl RenderBackend for CanvasRender {
                fn reset(&mut self) -> Result<()> {}

                fn flush(&mut self) -> Result<()> {}

                fn style(&mut self, _s: &ResolvedStyle) -> Result<()> {}

                fn text(&mut self, loc: Point, txt: &str) -> Result<()> {}

                fn supports_char_shift(&self) -> bool {}

                fn shift_chars(&mut self, _loc: Point, _count: i32) -> Result<()> {}
            }
        }

        pub mod buf {
            //! Buffer testing utilities.
            //! Utilities for working with TermBufs in tests.

            /// A struct for configuring buffer matching behavior. By default, it treats 'X' as a special
            /// marker for NULL cells in the buffer, allowing us to test partial renders.
            pub struct BufTest<'a> {}

            impl<'a> BufTest<'a> {
                /// Create a new BufTest with a reference to a TermBuf.
                pub fn new(buf: &'a TermBuf) -> Self {}

                /// Set the character used to match NULL cells in the buffer.
                /// Default is 'X'.
                pub fn with_null(self, null_char: char) -> Self {}

                /// Set a character that matches any character in the buffer.
                /// When set, this character in the expected pattern will match any character in the actual buffer.
                pub fn with_any(self, any_char: char) -> Self {}

                /// Returns true if the buffer content matches the expected lines.
                pub fn matches(&self, expected: &[&str]) -> bool {}

                /// Assert that the buffer matches the expected lines with pretty printed output on failure.
                pub fn assert_matches(&self, expected: &[&str]) {}

                /// Assert that the buffer matches the expected lines with pretty printed output on failure,
                /// with optional context information.
                pub fn assert_matches_with_context(
                    &self,
                    expected: &[&str],
                    context: Option<&str>,
                ) {
                }

                /// Does the buffer contain the supplied substring?
                pub fn contains_text(&self, txt: &str) -> bool {}

                /// Does the buffer contain the supplied substring in the given foreground colour?
                pub fn contains_text_fg(&self, txt: &str, fg: Color) -> bool {}

                /// Does the buffer contain the supplied substring with the given style?
                pub fn contains_text_style(&self, txt: &str, style: &PartialStyle) -> bool {}

                /// Dumps the contents of the buffer to the terminal for debugging purposes.
                pub fn dump(&self) {}

                /// Dumps a single line from the buffer to the terminal for debugging purposes.
                pub fn dump_line(&self, line_num: u32) {}

                /// Return the contents of a line as a `String`.
                pub fn line_text(&self, y: u32) -> Option<String> {}

                /// Return the contents of the buffer as lines of text.
                pub fn lines(&self) -> Vec<String> {}

                /// Return a newline-joined snapshot of the buffer contents.
                pub fn snapshot(&self) -> String {}
            }
        }

        pub mod dummyctx {
            //! Dummy context for tests.

            /// Dummy context for tests.
            #[derive(Default)]
            pub struct DummyContext {}

            impl ReadContext for DummyContext {
                fn node_id(&self) -> NodeId {}

                fn root_id(&self) -> NodeId {}

                fn view(&self) -> &View {}

                fn layout(&self) -> Layout {}

                fn node_view(&self, _node: NodeId) -> Option<View> {}

                fn node_type_id(&self, _node: NodeId) -> Option<TypeId> {}

                fn children_of(&self, _node: NodeId) -> Vec<NodeId> {}

                fn is_focused(&self) -> bool {}

                fn node_is_focused(&self, _node: NodeId) -> bool {}

                fn is_on_focus_path(&self) -> bool {}

                fn node_is_on_focus_path(&self, _node: NodeId) -> bool {}

                fn focus_path(&self, _root: NodeId) -> Path {}

                fn focused_leaf(&self, _root: NodeId) -> Option<NodeId> {}

                fn focusable_leaves(&self, _root: NodeId) -> Vec<NodeId> {}

                fn parent_of(&self, _node: NodeId) -> Option<NodeId> {}

                fn node_path(&self, _root: NodeId, _node: NodeId) -> Path {}

                fn child_keyed(&self, _key: &str) -> Option<NodeId> {}

                fn child_keyed_in(&self, _parent: NodeId, _key: &str) -> Option<NodeId> {}

                fn pending_help_snapshot(&self) -> Option<&OwnedHelpSnapshot> {}
            }

            impl Context for DummyContext {
                fn set_focus(&mut self, _node: NodeId) -> bool {}

                fn focus_dir_in(&mut self, _root: NodeId, _dir: Direction) {}

                fn focus_first_in(&mut self, _root: NodeId) {}

                fn focus_next_in(&mut self, _root: NodeId) {}

                fn focus_prev_in(&mut self, _root: NodeId) {}

                fn capture_mouse(&mut self) -> bool {}

                fn release_mouse(&mut self) -> bool {}

                fn scroll_to(&mut self, _x: u32, _y: u32) -> bool {}

                fn scroll_by(&mut self, _x: i32, _y: i32) -> bool {}

                fn invalidate_layout(&mut self) {}

                fn with_layout_of(
                    &mut self,
                    _node: NodeId,
                    _f: &mut dyn FnMut(&mut Layout),
                ) -> Result<()> {
                }

                fn create_detached_boxed(&mut self, _widget: Box<dyn Widget>) -> NodeId {}

                fn with_widget_mut(
                    &mut self,
                    _node: NodeId,
                    _f: &mut dyn FnMut(&mut dyn Widget, &mut dyn Context) -> Result<()>,
                ) -> Result<()> {
                }

                fn dispatch_command(
                    &mut self,
                    _cmd: &CommandInvocation,
                ) -> StdResult<ArgValue, CommandError> {
                }

                fn dispatch_command_scoped(
                    &mut self,
                    _frame: CommandScopeFrame,
                    _cmd: &CommandInvocation,
                ) -> StdResult<ArgValue, CommandError> {
                }

                fn current_event(&self) -> Option<&Event> {}

                fn current_mouse_event(&self) -> Option<MouseEvent> {}

                fn current_list_row(&self) -> Option<ListRowContext> {}

                fn add_child_to_boxed(
                    &mut self,
                    _parent: NodeId,
                    _widget: Box<dyn Widget>,
                ) -> Result<NodeId> {
                }

                fn add_child_to_keyed_boxed(
                    &mut self,
                    _parent: NodeId,
                    _key: &str,
                    _widget: Box<dyn Widget>,
                ) -> Result<NodeId> {
                }

                fn attach(&mut self, _parent: NodeId, _child: NodeId) -> Result<()> {}

                fn attach_keyed(
                    &mut self,
                    _parent: NodeId,
                    _key: &str,
                    _child: NodeId,
                ) -> Result<()> {
                }

                fn detach(&mut self, _child: NodeId) -> Result<()> {}

                fn remove_subtree(&mut self, _node: NodeId) -> Result<()> {}

                fn set_children_of(
                    &mut self,
                    _parent: NodeId,
                    _children: Vec<NodeId>,
                ) -> Result<()> {
                }

                fn set_hidden_of(&mut self, _node: NodeId, _hidden: bool) -> bool {}

                fn start(&mut self) -> Result<()> {}

                fn stop(&mut self) -> Result<()> {}

                fn exit(&mut self, _code: i32) {}

                fn push_effect(&mut self, _node: NodeId, _effect: Effect) -> Result<()> {}

                fn clear_effects(&mut self, _node: NodeId) -> Result<()> {}

                fn set_clear_inherited_effects(
                    &mut self,
                    _node: NodeId,
                    _clear: bool,
                ) -> Result<()> {
                }

                fn set_style(&mut self, _style: StyleMap) {}

                fn request_help_snapshot(&mut self, _target: NodeId) {}

                fn take_help_snapshot(&mut self) -> Option<OwnedHelpSnapshot> {}

                fn request_diagnostic_dump(&mut self, _target: NodeId) {}
            }
        }

        pub mod grid {
            //! Grid test helpers.
            //! Grid test utility for creating configurable grid layouts.

            /// A test utility for creating grids with configurable recursion and subdivisions.
            pub struct Grid {
                /// Root node for the grid.
                pub root: crate::NodeId,
            }

            impl Grid {
                /// Create a new grid with specified recursion levels and subdivisions per level.
                pub fn install(
                    core: &mut Core,
                    recursion: usize,
                    divisions: usize,
                ) -> Result<Self> {
                }

                /// Get the expected grid size in cells.
                pub fn expected_size(&self) -> Size {}

                /// Get the dimensions of the grid (number of cells in x and y).
                pub fn dimensions(&self) -> (usize, usize) {}

                /// Helper to find the deepest leaf node at a given position.
                pub fn find_leaf_at(&self, core: &Core, x: u32, y: u32) -> Option<String> {}
            }
        }

        pub mod harness {
            //! Harness for node testing.

            /// A simple harness that holds a [`Canopy`], a [`NopBackend`] backend and a
            /// root node ID. Tests drive the UI by sending key events and triggering renders
            /// and can then inspect the render buffer.
            pub struct Harness {
                /// The Canopy instance that manages the node tree and rendering.
                pub canopy: crate::Canopy,
                /// The backend used for rendering. In tests, this is a no-op backend.
                pub backend: super::render::NopBackend,
                /// The root node of the UI under test.
                pub root: crate::NodeId,
            }

            impl Harness {
                /// Create a harness builder for constructing a test harness with a fluent API.
                pub fn builder<W: Widget + Loader + 'static>(root: W) -> HarnessBuilder<W> {}

                /// Create a harness using `size` for the root layout.
                pub fn with_size<W: Widget + Loader + 'static>(
                    root: W,
                    size: Size,
                ) -> Result<Self> {
                }

                /// Create a harness with a default root size of 100x100.
                pub fn new<W: Widget + Loader + 'static>(root: W) -> Result<Self> {}

                /// Access the current render buffer. Panics if a render has not yet been performed.
                pub fn buf(&self) -> &TermBuf {}

                /// Send a key event and render.
                pub fn key<T>(&mut self, k: T) -> Result<()>
                where
                    T: Into<key::Key>, {
                }

                /// Send a mouse event and render.
                pub fn mouse(&mut self, m: mouse::MouseEvent) -> Result<()> {}

                /// Send a sequence of key events and render after each.
                pub fn keys<I, K>(&mut self, keys: I) -> Result<()>
                where
                    I: IntoIterator<Item = K>,
                    K: Into<key::Key>, {
                }

                /// Type a string as a sequence of key events.
                pub fn type_text(&mut self, text: &str) -> Result<()> {}

                /// Render the root node into the harness backend.
                pub fn render(&mut self) -> Result<()> {}

                /// Render and return a snapshot of the buffer contents.
                pub fn render_snapshot(&mut self) -> Result<String> {}

                /// Execute a script on the app under test.
                pub fn script(&mut self, script: &str) -> Result<()> {}

                /// Execute a closure with mutable access to a widget by node id.
                pub fn with_widget<W, R>(
                    &mut self,
                    node_id: impl Into<NodeId>,
                    f: impl FnOnce(&mut W) -> R,
                ) -> R
                where
                    W: Widget + 'static, {
                }

                /// Execute a closure with mutable access to the root widget.
                pub fn with_root_widget<W, R>(&mut self, f: impl FnOnce(&mut W) -> R) -> R
                where
                    W: Widget + 'static, {
                }

                /// Execute a closure with mutable access to the root widget and a context.
                pub fn with_root_context<W, R>(
                    &mut self,
                    f: impl FnMut(&mut W, &mut dyn Context) -> Result<R>,
                ) -> Result<R>
                where
                    W: Widget + 'static, {
                }

                /// Get a BufTest instance that references the current buffer.
                pub fn tbuf(&self) -> BufTest<'_> {}

                /// Find the first node whose path matches the filter, relative to the root.
                pub fn find_node(&self, path_filter: &str) -> Option<NodeId> {}

                /// Find all nodes whose paths match the filter, relative to the root.
                pub fn find_nodes(&self, path_filter: &str) -> Vec<NodeId> {}
            }

            /// Builder for creating a test harness with a fluent API.
            pub struct HarnessBuilder<W> {}

            impl<W: Widget + Loader + 'static> HarnessBuilder<W> {
                /// Set the size of the harness view.
                pub fn size(self, width: u32, height: u32) -> Self {}

                /// Build the harness with the configured settings.
                pub fn build(self) -> Result<Harness> {}
            }
        }

        pub mod render {
            //! Render helpers for tests.

            /// A dummy render backend that discards all output.
            /// This is useful for tests where we want to inspect the TermBuf directly.
            pub struct NopBackend;

            impl NopBackend {
                /// Construct a no-op backend.
                pub fn new() -> Self {}
            }

            impl RenderBackend for NopBackend {
                fn style(&mut self, _style: &ResolvedStyle) -> Result<()> {}

                fn text(&mut self, _loc: Point, _txt: &str) -> Result<()> {}

                fn supports_char_shift(&self) -> bool {}

                fn shift_chars(&mut self, _loc: Point, _count: i32) -> Result<()> {}

                fn flush(&mut self) -> Result<()> {}

                fn reset(&mut self) -> Result<()> {}
            }
        }

        pub mod ttree {
            //! Test tree helpers.
            //! This module defines a standard tree of instrumented nodes for testing.

            /// Thread-local state tracked by test nodes.
            #[derive(Debug, StructuralPartialEq, PartialEq, Eq, Clone)]
            pub struct State {
                /// Recorded event path entries.
                pub path: Vec<String>,
            }

            impl State {
                /// Construct a new empty state.
                pub fn new() -> Self {}

                /// Clear recorded events.
                pub fn reset(&mut self) {}

                /// Record a node event.
                pub fn add_event(&mut self, n: &NodeName, evt: &str, result: &EventOutcome) {}

                /// Record a command invocation.
                pub fn add_command(&mut self, n: &NodeName, cmd: &str) {}
            }

            /// Clear the global test state.
            pub fn reset_state() {}

            /// Get the current test state.
            pub fn get_state() -> State {}

            /// Allows tests to set the next event outcome on a node.
            pub trait OutcomeTarget {
                /// Set the next event outcome.
                fn set_outcome(&mut self, outcome: EventOutcome);
            }

            /// Test leaf node with instrumented behavior.
            pub struct BaLa {
                /// Next event outcome override.
                pub next_outcome: Option<EventOutcome>,
            }

            impl BaLa {
                /// Construct a new leaf node.
                pub fn new() -> Self {}

                /// A command that appears only on leaf nodes.
                pub fn c_leaf(&self, _core: &mut dyn Context) -> Result<()> {}

                /// Return a typed command reference for this command.
                pub fn cmd_c_leaf() -> &'static canopy::commands::CommandSpec {}
            }

            impl CommandNode for BaLa {
                fn commands() -> &'static [&'static canopy::commands::CommandSpec] {}
            }

            impl Widget for BaLa {
                fn accept_focus(&self, _ctx: &dyn ReadContext) -> bool {}

                fn render(&mut self, r: &mut Render<'_>, ctx: &dyn ReadContext) -> Result<()> {}

                fn on_event(
                    &mut self,
                    event: &Event,
                    _ctx: &mut dyn Context,
                ) -> Result<EventOutcome> {
                }

                fn name(&self) -> NodeName {}
            }

            impl OutcomeTarget for BaLa {
                fn set_outcome(&mut self, outcome: EventOutcome) {}
            }

            /// Test leaf node with instrumented behavior.
            pub struct BaLb {
                /// Next event outcome override.
                pub next_outcome: Option<EventOutcome>,
            }

            impl BaLb {
                /// Construct a new leaf node.
                pub fn new() -> Self {}

                /// A command that appears only on leaf nodes.
                pub fn c_leaf(&self, _core: &mut dyn Context) -> Result<()> {}

                /// Return a typed command reference for this command.
                pub fn cmd_c_leaf() -> &'static canopy::commands::CommandSpec {}
            }

            impl CommandNode for BaLb {
                fn commands() -> &'static [&'static canopy::commands::CommandSpec] {}
            }

            impl Widget for BaLb {
                fn accept_focus(&self, _ctx: &dyn ReadContext) -> bool {}

                fn render(&mut self, r: &mut Render<'_>, ctx: &dyn ReadContext) -> Result<()> {}

                fn on_event(
                    &mut self,
                    event: &Event,
                    _ctx: &mut dyn Context,
                ) -> Result<EventOutcome> {
                }

                fn name(&self) -> NodeName {}
            }

            impl OutcomeTarget for BaLb {
                fn set_outcome(&mut self, outcome: EventOutcome) {}
            }

            /// Test leaf node with instrumented behavior.
            pub struct BbLa {
                /// Next event outcome override.
                pub next_outcome: Option<EventOutcome>,
            }

            impl BbLa {
                /// Construct a new leaf node.
                pub fn new() -> Self {}

                /// A command that appears only on leaf nodes.
                pub fn c_leaf(&self, _core: &mut dyn Context) -> Result<()> {}

                /// Return a typed command reference for this command.
                pub fn cmd_c_leaf() -> &'static canopy::commands::CommandSpec {}
            }

            impl CommandNode for BbLa {
                fn commands() -> &'static [&'static canopy::commands::CommandSpec] {}
            }

            impl Widget for BbLa {
                fn accept_focus(&self, _ctx: &dyn ReadContext) -> bool {}

                fn render(&mut self, r: &mut Render<'_>, ctx: &dyn ReadContext) -> Result<()> {}

                fn on_event(
                    &mut self,
                    event: &Event,
                    _ctx: &mut dyn Context,
                ) -> Result<EventOutcome> {
                }

                fn name(&self) -> NodeName {}
            }

            impl OutcomeTarget for BbLa {
                fn set_outcome(&mut self, outcome: EventOutcome) {}
            }

            /// Test leaf node with instrumented behavior.
            pub struct BbLb {
                /// Next event outcome override.
                pub next_outcome: Option<EventOutcome>,
            }

            impl BbLb {
                /// Construct a new leaf node.
                pub fn new() -> Self {}

                /// A command that appears only on leaf nodes.
                pub fn c_leaf(&self, _core: &mut dyn Context) -> Result<()> {}

                /// Return a typed command reference for this command.
                pub fn cmd_c_leaf() -> &'static canopy::commands::CommandSpec {}
            }

            impl CommandNode for BbLb {
                fn commands() -> &'static [&'static canopy::commands::CommandSpec] {}
            }

            impl Widget for BbLb {
                fn accept_focus(&self, _ctx: &dyn ReadContext) -> bool {}

                fn render(&mut self, r: &mut Render<'_>, ctx: &dyn ReadContext) -> Result<()> {}

                fn on_event(
                    &mut self,
                    event: &Event,
                    _ctx: &mut dyn Context,
                ) -> Result<EventOutcome> {
                }

                fn name(&self) -> NodeName {}
            }

            impl OutcomeTarget for BbLb {
                fn set_outcome(&mut self, outcome: EventOutcome) {}
            }

            /// Test branch node with instrumented behavior.
            pub struct Ba {
                /// Next event outcome override.
                pub next_outcome: Option<EventOutcome>,
            }

            impl Ba {
                /// Construct a new branch node.
                pub fn new() -> Self {}
            }

            impl CommandNode for Ba {
                fn commands() -> &'static [&'static canopy::commands::CommandSpec] {}
            }

            impl Widget for Ba {
                fn accept_focus(&self, _ctx: &dyn ReadContext) -> bool {}

                fn render(&mut self, r: &mut Render<'_>, ctx: &dyn ReadContext) -> Result<()> {}

                fn on_event(
                    &mut self,
                    event: &Event,
                    _ctx: &mut dyn Context,
                ) -> Result<EventOutcome> {
                }

                fn name(&self) -> NodeName {}
            }

            impl OutcomeTarget for Ba {
                fn set_outcome(&mut self, outcome: EventOutcome) {}
            }

            /// Test branch node with instrumented behavior.
            pub struct Bb {
                /// Next event outcome override.
                pub next_outcome: Option<EventOutcome>,
            }

            impl Bb {
                /// Construct a new branch node.
                pub fn new() -> Self {}
            }

            impl CommandNode for Bb {
                fn commands() -> &'static [&'static canopy::commands::CommandSpec] {}
            }

            impl Widget for Bb {
                fn accept_focus(&self, _ctx: &dyn ReadContext) -> bool {}

                fn render(&mut self, r: &mut Render<'_>, ctx: &dyn ReadContext) -> Result<()> {}

                fn on_event(
                    &mut self,
                    event: &Event,
                    _ctx: &mut dyn Context,
                ) -> Result<EventOutcome> {
                }

                fn name(&self) -> NodeName {}
            }

            impl OutcomeTarget for Bb {
                fn set_outcome(&mut self, outcome: EventOutcome) {}
            }

            /// Root node for the test tree.
            pub struct R {
                /// Next event outcome override.
                pub next_outcome: Option<crate::widget::EventOutcome>,
            }

            impl R {
                /// Construct a new test root.
                pub fn new() -> Self {}

                /// A command that appears only on root.
                pub fn c_root(&self, _core: &mut dyn Context) -> Result<()> {}

                /// Return a typed command reference for this command.
                pub fn cmd_c_root() -> &'static canopy::commands::CommandSpec {}
            }

            impl CommandNode for R {
                fn commands() -> &'static [&'static canopy::commands::CommandSpec] {}
            }

            impl OutcomeTarget for R {
                fn set_outcome(&mut self, outcome: EventOutcome) {}
            }

            impl Widget for R {
                fn accept_focus(&self, _ctx: &dyn ReadContext) -> bool {}

                fn render(&mut self, r: &mut Render<'_>, ctx: &dyn ReadContext) -> Result<()> {}

                fn on_event(
                    &mut self,
                    event: &Event,
                    _ctx: &mut dyn Context,
                ) -> Result<EventOutcome> {
                }

                fn name(&self) -> NodeName {}
            }

            /// Node IDs for the test tree.
            #[derive(Debug, Clone, Copy)]
            pub struct TestTree {
                /// Root node id.
                pub root: crate::NodeId,
                /// Left branch node id.
                pub a: crate::NodeId,
                /// Right branch node id.
                pub b: crate::NodeId,
                /// Left-left leaf id.
                pub a_a: crate::NodeId,
                /// Left-right leaf id.
                pub a_b: crate::NodeId,
                /// Right-left leaf id.
                pub b_a: crate::NodeId,
                /// Right-right leaf id.
                pub b_b: crate::NodeId,
            }

            /// Run a function on our standard dummy app built from [`TestTree`].
            pub fn run_ttree(
                func: impl FnOnce(
                    &mut crate::Canopy,
                    crate::testing::backend::TestRender,
                    TestTree,
                ) -> crate::error::Result<()>,
            ) -> crate::error::Result<()> {
            }
        }
    }

    /// Callback marshalled onto the UI thread for live automation.
    pub type AutomationCallback = Box<dyn FnOnce(&mut Canopy) + Send + 'static>;

    /// Handle for submitting automation work to a live canopy runloop.
    #[derive(Clone)]
    pub struct AutomationHandle {}

    impl AutomationHandle {
        /// Queue a callback to run on the UI thread.
        pub fn submit(&self, callback: AutomationCallback) -> Result<()> {}

        /// Execute a closure on the UI thread and wait for its result.
        pub fn request<R, F>(&self, callback: F) -> Result<R>
        where
            R: Send + 'static,
            F: FnOnce(&mut Canopy) -> Result<R> + Send + 'static, {
        }
    }

    /// Monotonic identifier for a binding.
    #[derive(Debug, Clone, Copy, StructuralPartialEq, PartialEq, Eq, Hash)]
    pub struct BindingId(_);

    impl BindingId {
        /// Return the numeric binding identifier.
        pub fn as_u64(self) -> u64 {}

        /// Reconstruct a binding identifier from its numeric form.
        pub fn from_u64(id: u64) -> Self {}
    }

    /// Application runtime state and renderer coordination.
    pub struct Canopy {}

    impl Canopy {
        /// Construct a new Canopy instance.
        pub fn new() -> Self {}

        /// Return a handle for submitting automation work to this app's UI thread.
        pub fn automation_handle(&self) -> AutomationHandle {}

        /// Return the root node ID.
        pub fn root_id(&self) -> NodeId {}

        /// Create a detached widget node.
        pub fn create_detached<W>(&mut self, widget: W) -> TypedId<W>
        where
            W: Widget + 'static, {
        }

        /// Replace the root's children with a single node.
        pub fn set_root_child(&mut self, child: impl Into<NodeId>) -> Result<()> {}

        /// Return the active style map.
        pub fn style(&self) -> &StyleMap {}

        /// Mutate the active style map before the next render.
        pub fn style_mut(&mut self) -> &mut StyleMap {}

        /// Replace the active style map before the next render.
        pub fn set_style(&mut self, style: StyleMap) {}

        /// Register a backend controller.
        pub fn register_backend<T: BackendControl + 'static>(&mut self, be: T) {}

        /// Get a reference to the current render buffer, if any.
        pub fn buf(&self) -> Option<&TermBuf> {}

        /// Run a compiled script by id on the target node.
        pub fn run_script(
            &mut self,
            node_id: impl Into<NodeId>,
            sid: script::ScriptId,
        ) -> Result<()> {
        }

        /// Compile a script and return its identifier.
        pub fn compile_script(&mut self, source: &str) -> Result<script::ScriptId> {}

        /// Evaluate a Luau source string in the current app context.
        pub fn eval_script(&mut self, source: &str) -> Result<()> {}

        /// Evaluate a Luau source string and return its value.
        pub fn eval_script_value(&mut self, source: &str) -> Result<commands::ArgValue> {}

        /// Evaluate a Luau source string with a cooperative timeout.
        pub fn eval_script_value_with_timeout(
            &mut self,
            source: &str,
            timeout: Duration,
        ) -> Result<commands::ArgValue> {
        }

        /// Evaluate the app's built-in default bindings script.
        pub fn run_default_script(&mut self, source: &str) -> Result<()> {}

        /// Register a Luau script as the default bindings for a widget namespace.
        pub fn register_default_bindings(&mut self, name: &str, script: &str) -> Result<()> {}

        /// Register a named fixture available to headless and live automation.
        pub fn register_fixture(&mut self, fixture: Fixture) -> Result<()> {}

        /// Return registered fixture metadata in stable name order.
        pub fn fixture_infos(&self) -> Vec<FixtureInfo> {}

        /// Apply a named fixture to the current app instance.
        pub fn apply_fixture(&mut self, name: &str) -> Result<()> {}

        /// Run a closure against the root context.
        pub fn with_root_context<R>(
            &mut self,
            f: impl FnOnce(&mut dyn crate::Context) -> Result<R>,
        ) -> Result<R> {
        }

        /// Type-check a Luau source string against the finalized app API.
        pub fn check_script(&mut self, source: &str) -> Result<script::ScriptCheckResult> {}

        /// Drain and return log lines recorded by the most recent script evaluation.
        pub fn take_script_logs(&self) -> Vec<String> {}

        /// Drain and return assertion outcomes from the most recent script evaluation.
        pub fn take_script_assertions(&self) -> Vec<script::ScriptAssertion> {}

        /// Evaluate a Luau config file from disk.
        pub fn run_config(&mut self, path: &FsPath) -> Result<()> {}

        /// Remove a binding by ID. Returns true if a binding was removed.
        pub fn unbind(&mut self, id: inputmap::BindingId) -> bool {}

        /// Remove bindings for a key input, optionally filtered by mode and path.
        pub fn unbind_key_input<K>(
            &mut self,
            key: K,
            mode: Option<&str>,
            path_filter: Option<&str>,
        ) -> usize
        where
            key::Key: From<K>, {
        }

        /// Remove bindings for a mouse input, optionally filtered by mode and path.
        pub fn unbind_mouse_input<K>(
            &mut self,
            mouse: K,
            mode: Option<&str>,
            path_filter: Option<&str>,
        ) -> usize
        where
            mouse::Mouse: From<K>, {
        }

        /// Remove all bindings from all modes.
        pub fn clear_bindings(&mut self) -> usize {}

        /// Return all bindings defined for a mode.
        pub fn bindings_for_mode(&self, mode: &str) -> Vec<inputmap::BindingInfo<'_>> {}

        /// Return bindings in a mode that match a specific path.
        pub fn bindings_matching_path(
            &self,
            mode: &str,
            path: &Path,
        ) -> Vec<inputmap::MatchedBindingInfo<'_>> {
        }

        /// Return the active input mode.
        pub fn input_mode(&self) -> &str {}

        /// Set the active input mode.
        pub fn set_input_mode(&mut self, mode: &str) -> Result<()> {}

        /// Bind a key or mouse input to switch the active input mode.
        pub fn bind_input_mode(
            &mut self,
            mode: &str,
            input: inputmap::InputSpec,
            path_filter: &str,
            next_mode: &str,
        ) -> Result<inputmap::BindingId> {
        }

        /// Return the most recent key or mouse route trace.
        pub fn route_trace(&self) -> &[RouteTraceEntry] {}

        /// Load the commands from a command node using the default node name.
        /// Returns an error if any command id is already registered.
        pub fn add_commands<T: commands::CommandNode>(&mut self) -> Result<()> {}

        /// Finalize the script API surface for this app.
        pub fn finalize_api(&mut self) -> Result<()> {}

        /// Return the rendered Luau definition file for this app.
        pub fn script_api(&self) -> &str {}

        /// Output a formatted table of commands to a writer.
        ///
        /// If `include_hidden` is false, commands with `doc.hidden = true` are excluded.
        pub fn print_command_table(&self, w: &mut dyn Write, include_hidden: bool) -> Result<()> {}

        /// Return command availability from the current focus position.
        ///
        /// This computes which commands would resolve to a target if dispatched from the current
        /// focus. For each command:
        /// - Free commands always have `resolution = Some(Free)`
        /// - Node-routed commands have `resolution = Some(Subtree{..})` or `Some(Ancestor{..})`
        ///   if a matching node exists, `None` otherwise
        pub fn command_availability_from_focus(&self) -> Vec<commands::CommandAvailability<'_>> {}

        /// Return command availability from a specific node.
        ///
        /// Computes which commands would dispatch to a target, using the same resolution logic
        /// as `commands::dispatch`:
        /// 1. First search the subtree rooted at `start` in pre-order
        /// 2. Then walk ancestors
        pub fn command_availability_from_node(
            &self,
            start: NodeId,
        ) -> Vec<commands::CommandAvailability<'_>> {
        }

        /// Generate a contextual help snapshot for the current focus.
        ///
        /// The snapshot includes:
        /// - Bindings that would match from the focus path
        /// - Commands with their availability status
        pub fn help_snapshot(&self) -> super::help::HelpSnapshot<'_> {}

        /// Build a diagnostic dump with tree, focus, and binding details.
        pub fn diagnostic_dump(&self, target: NodeId) -> String {}

        /// Has the focus path status of this node changed since the last render sweep?
        pub fn node_focus_path_changed(&self, node_id: impl Into<NodeId>) -> bool {}

        /// Render the widget tree. All visible nodes are rendered.
        pub fn render<R: RenderBackend>(&mut self, be: &mut R) -> Result<()> {}

        /// Set the size on the root node.
        pub fn set_root_size(&mut self, size: Size) -> Result<()> {}
    }

    /// A typed key for keyed children.
    ///
    /// This trait associates a string key with a specific widget type, providing
    /// compile-time type safety for keyed child access.
    ///
    /// Use the [`key!`] macro to define keys:
    ///
    /// ```ignore
    /// // Key with same name as widget type
    /// Editor);
    ///
    /// // Key with custom name
    /// ModalSlot: Modal);
    /// ```
    pub trait ChildKey {
        type Widget: Widget + 'static;
        const KEY: &'static str;
    }

    /// Command dispatch context helpers.
    pub trait CommandContext: Context {
        /// Dispatch a prepared command invocation from the current node.
        fn dispatch_prepared_command(
            &mut self,
            command: &CommandInvocation,
        ) -> StdResult<ArgValue, CommandError> {
        }
    }

    /// Mutable context available to widgets during event handling.
    pub trait Context: ReadContext {
        /// Focus a node. Returns `true` if focus changed.
        fn set_focus(&mut self, node: NodeId) -> bool;

        /// Move focus in a specified direction within the current node's subtree.
        fn focus_dir(&mut self, dir: Direction) {}

        /// Move focus in a specified direction within the specified subtree.
        fn focus_dir_in(&mut self, root: NodeId, dir: Direction);

        /// Move focus in a specified direction within the entire tree (from root).
        fn focus_dir_global(&mut self, dir: Direction) {}

        /// Focus the first node that accepts focus in the current node's subtree.
        fn focus_first(&mut self) {}

        /// Focus the first node that accepts focus in the specified subtree.
        fn focus_first_in(&mut self, root: NodeId);

        /// Focus the first node that accepts focus in the entire tree (from root).
        fn focus_first_global(&mut self) {}

        /// Focus the next node in the current node's subtree.
        fn focus_next(&mut self) {}

        /// Focus the next node in the specified subtree.
        fn focus_next_in(&mut self, root: NodeId);

        /// Focus the next node in the entire tree (from root).
        fn focus_next_global(&mut self) {}

        /// Focus the previous node in the current node's subtree.
        fn focus_prev(&mut self) {}

        /// Focus the previous node in the specified subtree.
        fn focus_prev_in(&mut self, root: NodeId);

        /// Focus the previous node in the entire tree (from root).
        fn focus_prev_global(&mut self) {}

        /// Move focus to the right within the current node's subtree.
        fn focus_right(&mut self) {}

        /// Move focus to the right within the specified subtree.
        fn focus_right_in(&mut self, root: NodeId) {}

        /// Move focus to the right within the entire tree (from root).
        fn focus_right_global(&mut self) {}

        /// Move focus to the left within the current node's subtree.
        fn focus_left(&mut self) {}

        /// Move focus to the left within the specified subtree.
        fn focus_left_in(&mut self, root: NodeId) {}

        /// Move focus to the left within the entire tree (from root).
        fn focus_left_global(&mut self) {}

        /// Move focus upward within the current node's subtree.
        fn focus_up(&mut self) {}

        /// Move focus upward within the specified subtree.
        fn focus_up_in(&mut self, root: NodeId) {}

        /// Move focus upward within the entire tree (from root).
        fn focus_up_global(&mut self) {}

        /// Move focus downward within the current node's subtree.
        fn focus_down(&mut self) {}

        /// Move focus downward within the specified subtree.
        fn focus_down_in(&mut self, root: NodeId) {}

        /// Move focus downward within the entire tree (from root).
        fn focus_down_global(&mut self) {}

        /// Capture mouse events for the current node. Returns `true` if capture changed.
        fn capture_mouse(&mut self) -> bool;

        /// Release mouse capture if held by the current node. Returns `true` if capture changed.
        fn release_mouse(&mut self) -> bool;

        /// Scroll the view to the specified position. Returns `true` if movement occurred.
        fn scroll_to(&mut self, x: u32, y: u32) -> bool;

        /// Scroll the view by the given offsets. Returns `true` if movement occurred.
        fn scroll_by(&mut self, x: i32, y: i32) -> bool;

        /// Scroll the view up by one page. Returns `true` if movement occurred.
        fn page_up(&mut self) -> bool {}

        /// Scroll the view down by one page. Returns `true` if movement occurred.
        fn page_down(&mut self) -> bool {}

        /// Scroll the view up by one line. Returns `true` if movement occurred.
        fn scroll_up(&mut self) -> bool {}

        /// Scroll the view down by one line. Returns `true` if movement occurred.
        fn scroll_down(&mut self) -> bool {}

        /// Scroll the view left by one line. Returns `true` if movement occurred.
        fn scroll_left(&mut self) -> bool {}

        /// Scroll the view right by one line. Returns `true` if movement occurred.
        fn scroll_right(&mut self) -> bool {}

        /// Mark this node dirty so the next frame re-runs layout.
        fn invalidate_layout(&mut self);

        /// Update the layout for the current node.
        fn with_layout(&mut self, f: &mut dyn FnMut(&mut Layout)) -> Result<()> {}

        /// Update the layout for a specific node.
        fn with_layout_of(&mut self, node: NodeId, f: &mut dyn FnMut(&mut Layout)) -> Result<()>;

        /// Create a new widget node detached from the tree.
        fn create_detached_boxed(&mut self, widget: Box<dyn Widget>) -> NodeId;

        /// Execute a closure with mutable access to a widget and its node-bound context.
        fn with_widget_mut(
            &mut self,
            node: NodeId,
            f: &mut dyn FnMut(&mut dyn Widget, &mut dyn Context) -> Result<()>,
        ) -> Result<()>;

        /// Dispatch a command relative to this node.
        fn dispatch_command(
            &mut self,
            cmd: &CommandInvocation,
        ) -> StdResult<ArgValue, CommandError>;

        /// Dispatch a command with an explicit command-scope frame.
        fn dispatch_command_scoped(
            &mut self,
            frame: CommandScopeFrame,
            cmd: &CommandInvocation,
        ) -> StdResult<ArgValue, CommandError>;

        /// Return the current event snapshot for injection.
        fn current_event(&self) -> Option<&Event>;

        /// Return the current mouse event for injection.
        fn current_mouse_event(&self) -> Option<MouseEvent>;

        /// Return the current list-row context for injection.
        fn current_list_row(&self) -> Option<ListRowContext>;

        /// Add a boxed widget as a child of a specific parent and return the new node ID.
        fn add_child_to_boxed(&mut self, parent: NodeId, widget: Box<dyn Widget>)
            -> Result<NodeId>;

        /// Add a boxed widget as a keyed child of a specific parent and return the new node ID.
        fn add_child_to_keyed_boxed(
            &mut self,
            parent: NodeId,
            key: &str,
            widget: Box<dyn Widget>,
        ) -> Result<NodeId>;

        /// Attach a detached child to a parent.
        fn attach(&mut self, parent: NodeId, child: NodeId) -> Result<()>;

        /// Attach a detached child to a parent using a unique key.
        fn attach_keyed(&mut self, parent: NodeId, key: &str, child: NodeId) -> Result<()>;

        /// Detach a child from its parent.
        fn detach(&mut self, child: NodeId) -> Result<()>;

        /// Remove a node and all descendants from the arena.
        fn remove_subtree(&mut self, node: NodeId) -> Result<()>;

        /// Replace the children list for the current node.
        fn set_children(&mut self, children: Vec<NodeId>) -> Result<()> {}

        /// Replace the children list for a specific parent node.
        fn set_children_of(&mut self, parent: NodeId, children: Vec<NodeId>) -> Result<()>;

        /// Set the current node's visibility. Returns `true` if visibility changed.
        fn set_hidden(&mut self, hidden: bool) -> bool {}

        /// Set a specific node's visibility. Returns `true` if visibility changed.
        fn set_hidden_of(&mut self, node: NodeId, hidden: bool) -> bool;

        /// Hide the current node. Returns `true` if visibility changed.
        fn hide(&mut self) -> bool {}

        /// Hide a specific node. Returns `true` if visibility changed.
        fn hide_node(&mut self, node: NodeId) -> bool {}

        /// Show the current node. Returns `true` if visibility changed.
        fn show(&mut self) -> bool {}

        /// Show a specific node. Returns `true` if visibility changed.
        fn show_node(&mut self, node: NodeId) -> bool {}

        /// Start the backend renderer.
        fn start(&mut self) -> Result<()>;

        /// Stop the backend renderer, releasing control of the terminal.
        fn stop(&mut self) -> Result<()>;

        /// Request a cooperative shutdown with the provided status code.
        fn exit(&mut self, code: i32);

        /// Add an effect to a node that will be applied during rendering.
        /// Effects stack and inherit through the tree.
        fn push_effect(&mut self, node: NodeId, effect: Effect) -> Result<()>;

        /// Clear all effects on a node.
        fn clear_effects(&mut self, node: NodeId) -> Result<()>;

        /// Set whether a node should clear inherited effects before applying local ones.
        fn set_clear_inherited_effects(&mut self, node: NodeId, clear: bool) -> Result<()>;

        /// Set the style map to be used for rendering.
        /// The style change will be applied before the next render.
        fn set_style(&mut self, style: StyleMap);

        /// Request a help snapshot to be injected into the specified target node.
        ///
        /// This should be called before changing focus or layout, so the snapshot
        /// captures the pre-help context. After the current command returns, Canopy
        /// will capture the snapshot and inject it into the target widget.
        fn request_help_snapshot(&mut self, target: NodeId);

        /// Take the pending help snapshot, if any.
        ///
        /// This is called by help widgets to retrieve the snapshot that was
        /// captured when `request_help_snapshot` was called. Returns `None` if
        /// no snapshot is pending.
        fn take_help_snapshot(&mut self) -> Option<OwnedHelpSnapshot>;

        /// Request a diagnostic dump for a target node.
        fn request_diagnostic_dump(&mut self, target: NodeId);
    }

    /// A named, reproducible application state.
    #[derive(Clone)]
    pub struct Fixture {
        /// Fixture name.
        pub name: String,
        /// Human-readable fixture description.
        pub description: String,
        /// Setup closure applied to the current canopy instance.
        pub setup: std::sync::Arc<
            dyn Fn(&mut super::canopy::Canopy) -> crate::error::Result<()> + Send + Sync,
        >,
    }

    impl Fixture {
        /// Construct a fixture from owned name/description values.
        pub fn new(
            name: impl Into<String>,
            description: impl Into<String>,
            setup: impl Fn(&mut Canopy) -> Result<()> + Send + Sync + 'static,
        ) -> Self {
        }

        /// Return fixture metadata without the setup closure.
        pub fn info(&self) -> FixtureInfo {}
    }

    /// Serializable metadata about a registered fixture.
    #[derive(
        Serialize, Debug, Clone, StructuralPartialEq, PartialEq, Eq, Serialize, Deserialize,
    )]
    pub struct FixtureInfo {
        /// Fixture name.
        pub name: String,
        /// Human-readable fixture description.
        pub description: String,
    }

    impl JsonSchema for FixtureInfo {
        fn schema_name() -> schemars::_private::alloc::borrow::Cow<'static, str> {}

        fn schema_id() -> schemars::_private::alloc::borrow::Cow<'static, str> {}

        fn json_schema(generator: &mut schemars::SchemaGenerator) -> schemars::Schema {}

        fn inline_schema() -> bool {}
    }

    /// Focus-related context helpers.
    pub trait FocusContext: Context {
        /// Focus a specific node.
        fn focus_node(&mut self, node: NodeId) -> bool {}

        /// Move focus from the current node in a direction.
        fn move_focus(&mut self, direction: Direction) {}
    }

    /// Ordered keyed child collection helper.
    ///
    /// Stores a stable mapping from keys to node IDs plus a current order. Use
    /// [`KeyedChildren::reconcile`] to create, update, and reorder children based on a desired key list.
    #[derive(Debug, Default)]
    pub struct KeyedChildren<K> {}

    impl<K> KeyedChildren<K>
    where
        K: Eq + Hash + Clone,
    {
        /// Construct an empty keyed collection.
        pub fn new() -> Self {}

        /// Return true if there are no ordered keys.
        pub fn is_empty(&self) -> bool {}

        /// Return the number of ordered keys.
        pub fn len(&self) -> usize {}

        /// Return the ordered key slice.
        pub fn keys(&self) -> &[K] {}

        /// Return the key at a given index.
        pub fn key_at(&self, index: usize) -> Option<&K> {}

        /// Return the node ID for a key, if present.
        pub fn id_for(&self, key: &K) -> Option<NodeId> {}

        /// Return the node ID at a given index, if present.
        pub fn id_at(&self, index: usize) -> Option<NodeId> {}

        /// Iterate node IDs in the current order.
        pub fn iter_ids(&self) -> impl Iterator<Item = NodeId> + '_ {}

        /// Reconcile this collection against the desired key order.
        pub fn reconcile<W, I, C, U>(
            &mut self,
            ctx: &mut dyn Context,
            desired: I,
            create: C,
            update: U,
            remove: RemovePolicy,
        ) -> Result<Vec<TypedId<W>>>
        where
            W: Widget + 'static,
            I: IntoIterator<Item = K>,
            C: FnMut(&K) -> W,
            U: FnMut(&K, TypedId<W>, &mut dyn Context) -> Result<()>, {
        }

        /// Reconcile this collection against the desired key order with fallible creation.
        pub fn try_reconcile<W, I, C, U>(
            &mut self,
            ctx: &mut dyn Context,
            desired: I,
            create: C,
            update: U,
            remove: RemovePolicy,
        ) -> Result<Vec<TypedId<W>>>
        where
            W: Widget + 'static,
            I: IntoIterator<Item = K>,
            C: FnMut(&K) -> Result<W>,
            U: FnMut(&K, TypedId<W>, &mut dyn Context) -> Result<()>, {
        }
    }

    /// Layout mutation context helpers.
    pub trait LayoutContext: Context {
        /// Replace a node's layout.
        fn replace_layout(&mut self, node: impl Into<NodeId>, layout: Layout) -> Result<()> {}
    }

    /// Validate a child view position against the parent canvas bounds.
    /// A trait that allows widgets to perform recursive initialization of themselves and their
    /// children.
    pub trait Loader {
        /// Load commands or resources into the canopy instance.
        /// Returns an error if loading fails.
        fn load(_: &mut Canopy) -> Result<()> {}
    }

    /// Opaque identifier for a node stored in the Core arena.
    #[derive(
        Copy, Clone, Default, Eq, StructuralPartialEq, PartialEq, Ord, PartialOrd, Hash, Debug,
    )]
    pub struct NodeId(_);

    impl From<KeyData> for NodeId {
        fn from(k: KeyData) -> Self {}
    }

    impl Key for NodeId {
        fn data(&self) -> KeyData {}
    }

    impl<T> From<TypedId<T>> for NodeId {
        fn from(value: TypedId<T>) -> Self {}
    }

    /// A path of node name components.
    #[derive(Debug, Clone, StructuralPartialEq, PartialEq, Eq, FromStr, Display)]
    pub struct Path {}

    impl Path {
        /// Construct an empty path.
        pub fn empty() -> Self {}

        /// Parse and validate a path from a slash-separated string.
        pub fn parse(path: &str) -> Result<Self> {}

        /// Pop an item off the end of the path, modifying it in place. Return None
        /// if the path is empty.
        pub fn pop(&mut self) -> Option<String> {}

        /// Construct a path from a slice of components.
        pub fn new<I>(v: I) -> Self
        where
            I: IntoIterator,
            I::Item: AsRef<str>, {
        }
    }

    impl From<Vec<String>> for Path {
        fn from(path: Vec<String>) -> Self {}
    }

    impl From<&[&str]> for Path {
        fn from(v: &[&str]) -> Self {}
    }

    impl From<&str> for Path {
        fn from(v: &str) -> Self {}
    }

    /// A validated path filter used to search node paths.
    ///
    /// Filters support `*` for one component and `**` for zero or more components.
    /// Literal components must be valid [`NodeName`] values.
    #[derive(Debug, Clone, FromStr)]
    pub struct PathFilter {}

    impl PathFilter {
        /// Compile a validated path filter.
        pub fn new(filter: &str) -> Result<Self> {}

        /// Compile a filter after normalizing it to a full-path match.
        pub fn normalized(filter: &str) -> Result<Self> {}

        /// Return the original filter string.
        pub fn as_str(&self) -> &str {}
    }

    /// Read-only context available to widgets during render and measure.
    pub trait ReadContext {
        /// The node currently being rendered.
        fn node_id(&self) -> NodeId;

        /// The root node of the tree.
        fn root_id(&self) -> NodeId;

        /// View information for the current node.
        fn view(&self) -> &View;

        /// Cached layout configuration for the current node.
        fn layout(&self) -> Layout;

        /// View information for a specific node.
        fn node_view(&self, node: NodeId) -> Option<View>;

        /// Widget type identifier for a specific node.
        fn node_type_id(&self, node: NodeId) -> Option<TypeId>;

        /// Canvas size for the current node.
        fn canvas(&self) -> Size {}

        /// Visible view rectangle in content coordinates.
        fn view_rect(&self) -> Rect {}

        /// Visible view rectangle in local outer coordinates.
        fn view_rect_local(&self) -> Rect {}

        /// Local outer rectangle for this node.
        fn outer_rect_local(&self) -> Rect {}

        /// Children of the current node in tree order.
        fn children(&self) -> Vec<NodeId> {}

        /// Children of a specific node in tree order.
        fn children_of(&self, node: NodeId) -> Vec<NodeId>;

        /// Does the current node have focus?
        fn is_focused(&self) -> bool;

        /// Does the specified node have focus?
        fn node_is_focused(&self, node: NodeId) -> bool;

        /// Is the current node on the focus path?
        fn is_on_focus_path(&self) -> bool;

        /// Is the specified node on the focus path?
        fn node_is_on_focus_path(&self, node: NodeId) -> bool;

        /// Return the focus path for the subtree under `root`.
        fn focus_path(&self, root: NodeId) -> Path;

        /// Return the focused leaf under the subtree rooted at `root`.
        fn focused_leaf(&self, root: NodeId) -> Option<NodeId>;

        /// Return focusable leaves in pre-order under the subtree rooted at `root`.
        fn focusable_leaves(&self, root: NodeId) -> Vec<NodeId>;

        /// Return the parent of a node, or `None` if it is the root or not found.
        fn parent_of(&self, node: NodeId) -> Option<NodeId>;

        /// Return the path for a node relative to a root.
        fn node_path(&self, root: NodeId, node: NodeId) -> Path;

        /// Return a keyed child relative to the current node.
        fn child_keyed(&self, key: &str) -> Option<NodeId>;

        /// Return a keyed child relative to a specific parent node.
        fn child_keyed_in(&self, parent: NodeId, key: &str) -> Option<NodeId>;

        /// Current focus generation counter.
        fn current_focus_gen(&self) -> u64 {}

        /// Find the first node whose path matches the filter, relative to the current node.
        ///
        /// The filter is normalized to match full paths.
        fn find_node(&self, path_filter: &str) -> Option<NodeId> {}

        /// Find the first node whose path matches the validated filter.
        fn find_node_matching(&self, path_filter: &PathFilter) -> Option<NodeId> {}

        /// Find all nodes whose paths match the filter, relative to the current node.
        ///
        /// The filter is normalized to match full paths.
        fn find_nodes(&self, path_filter: &str) -> Vec<NodeId> {}

        /// Find all nodes whose paths match the validated filter.
        fn find_nodes_matching(&self, path_filter: &PathFilter) -> Vec<NodeId> {}

        /// Peek at the pending help snapshot, if any.
        ///
        /// This is used by help widgets to check if a snapshot is available
        /// during render, without consuming it.
        fn pending_help_snapshot(&self) -> Option<&OwnedHelpSnapshot>;
    }

    /// Policy for removing children that are no longer desired.
    #[derive(Debug, Clone, Copy, StructuralPartialEq, PartialEq, Eq)]
    pub enum RemovePolicy {
        /// Detach nodes from the tree but keep them alive.
        Detach,
        /// Remove nodes and their descendants from the arena.
        RemoveSubtree,
        /// Hide nodes and keep them available for reuse.
        Hide,
    }

    /// Scroll context helpers.
    pub trait ScrollContext: Context {
        /// Scroll to a typed point.
        fn scroll_to_point(&mut self, point: Point) -> bool {}
    }

    /// Slot helper for keyed children that caches the resolved typed ID.
    #[derive(Debug, Default)]
    pub struct Slot<K: ChildKey> {}

    impl<K: ChildKey> Slot<K> {
        /// Construct an empty slot.
        pub fn new() -> Self {}

        /// Clear any cached typed ID.
        pub fn clear(&mut self) {}

        /// Get or create the keyed child under the current node.
        pub fn get_or_create(
            &mut self,
            ctx: &mut dyn Context,
            make: impl FnOnce() -> K::Widget,
        ) -> Result<TypedId<K::Widget>> {
        }

        /// Get or create the keyed child under a specific parent node.
        pub fn get_or_create_in(
            &mut self,
            ctx: &mut dyn Context,
            parent: impl Into<NodeId>,
            make: impl FnOnce() -> K::Widget,
        ) -> Result<TypedId<K::Widget>> {
        }

        /// Execute a closure with a keyed child under the current node.
        pub fn with<R>(
            &mut self,
            ctx: &mut dyn Context,
            f: impl FnOnce(&mut K::Widget, &mut dyn Context) -> Result<R>,
        ) -> Result<R> {
        }

        /// Execute a closure with a keyed child under a specific parent node.
        pub fn with_in<R>(
            &mut self,
            ctx: &mut dyn Context,
            parent: impl Into<NodeId>,
            f: impl FnOnce(&mut K::Widget, &mut dyn Context) -> Result<R>,
        ) -> Result<R> {
        }
    }

    /// Style context helpers.
    pub trait StyleContext: Context {
        /// Queue a style map for the next render pass.
        fn replace_style(&mut self, style: StyleMap) {}
    }

    /// Tree mutation context helpers.
    pub trait TreeContext: Context {
        /// Add a typed child to a parent node.
        fn add_child_widget<W: Widget + 'static>(
            &mut self,
            parent: impl Into<NodeId>,
            widget: W,
        ) -> Result<TypedId<W>> {
        }

        /// Remove a subtree rooted at `node`.
        fn remove_node_subtree(&mut self, node: impl Into<NodeId>) -> Result<()> {}
    }

    /// Type-safe wrapper around a node identifier tied to a widget type.
    #[derive(Debug, StructuralPartialEq, PartialEq, Eq, Hash, Clone, Copy)]
    pub struct TypedId<T> {}

    impl<T> TypedId<T> {
        /// Wrap an untyped node identifier.
        pub fn new(id: NodeId) -> Self {}
    }

    impl<T> From<TypedId<T>> for NodeId {
        fn from(value: TypedId<T>) -> Self {}
    }

    pub use canopy_derive::command;
    pub use canopy_derive::derive_commands;
    pub use canopy_derive::CommandArg;
    pub use canopy_derive::CommandEnum;
    /// The result of an event handler.
    #[derive(Debug, StructuralPartialEq, PartialEq, Eq, Clone)]
    pub enum EventOutcome {
        /// The event was processed and propagation stops.
        Handle,
        /// The event was processed without a state change and propagation stops.
        Consume,
        /// The event was not handled and will bubble up the tree.
        Ignore,
    }

    /// Widgets are the behavior attached to nodes in the Core arena.
    pub trait Widget: Any + Send {
        /// Layout configuration for this widget.
        fn layout(&self) -> Layout {}

        /// Measure intrinsic content size (content box, excludes Layout padding).
        fn measure(&self, c: MeasureConstraints) -> Measurement {}

        /// Canvas size in content coordinates (for scrolling).
        ///
        /// `view` is this node's content size (outer minus padding).
        fn canvas(&self, view: Size<u32>, _ctx: &CanvasContext<'_>) -> Size<u32> {}

        /// Render this widget's own content. Does not render children.
        fn render(&mut self, _frame: &mut Render<'_>, _ctx: &dyn ReadContext) -> Result<()> {}

        /// Handle events.
        fn on_event(&mut self, _event: &Event, _ctx: &mut dyn Context) -> Result<EventOutcome> {}

        /// Attempt to focus this widget.
        ///
        /// Widgets can use the provided context to query their tree state (e.g., whether they have
        /// children) when deciding whether to accept focus.
        fn accept_focus(&self, _ctx: &dyn ReadContext) -> bool {}

        /// Cursor specification for focused widgets.
        fn cursor(&self) -> Option<cursor::Cursor> {}

        /// Scheduled poll endpoint.
        fn poll(&mut self, _ctx: &mut dyn Context) -> Option<Duration> {}

        /// Called exactly once when the widget is first mounted in the tree, before the first render.
        ///
        /// The framework guarantees single invocation via an internal `mounted` flag on each node.
        /// There is no need to guard against multiple calls within this method.
        fn on_mount(&mut self, _ctx: &mut dyn Context) -> Result<()> {}

        /// Validation hook before a node is removed from the arena.
        ///
        /// This hook must be side-effect free or safely repeatable.
        fn pre_remove(&mut self, _ctx: &mut dyn Context) -> Result<()> {}

        /// Called exactly once immediately before the node is removed from the arena.
        fn on_unmount(&mut self, _ctx: &mut dyn Context) {}

        /// Name used for commands and paths.
        fn name(&self) -> NodeName {}
    }

    /// Convenience macro for building named arguments.
    #[macro_export]
    macro_rules! named_args {
    ($($key:ident : $value:expr),* $(,)?) => { ... };
}
    /// Macro to create a Color from a hex string at compile time
    #[macro_export]
    macro_rules! rgb {
    ($hex:literal) => { ... };
}
    /// A helper macro to create buffers for the termbuf match assertions.
    #[macro_export]
    macro_rules! buf {
    ($($line:literal)*) => { ... };
}
    /// Define a typed key for keyed children.
    ///
    /// # Examples
    ///
    /// ```ignore
    /// // Simple form: key name matches widget type, string key is snake_case
    /// Editor);  // KEY = "Editor", Widget = Editor (private)
    /// pub Editor);  // same, but public
    ///
    /// // Custom name form: specify the widget type explicitly
    /// ModalSlot: Modal);  // KEY = "ModalSlot", Widget = Modal (private)
    /// pub ModalSlot: Modal);  // same, but public
    /// ```
    #[macro_export]
    macro_rules! key {
    ($vis:vis $name:ident) => { ... };
    ($vis:vis $name:ident : $widget:ty) => { ... };
}
}

