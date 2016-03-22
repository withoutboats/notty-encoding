use args::{Argument, SplitKind, SaveGrid, ResizeRule};
use cmds::EscCode;

/// Add a new panel to the top of a section of the screen. This panel will contain an empty grid.
/// Argument 1: Which section to put the panel over (defaults to the active section).
/// Argument 2: Whether or not the new grid created retains offscreen state (defaults to true).
pub struct PushPanel(pub Option<u64>, pub Option<bool>);

impl EscCode for PushPanel {
    const OPCODE: u16 = 0x60;
    fn args(&self) -> Vec<String> {
        encode_args![?self.0, ?self.1]
    }
}

/// Remove the top panel in a section of the screen. If that section has only one panel, this does
/// nothing.
/// Argument 1: Which section to remove the panel from (defaults to the active section).
pub struct PopPanel(pub Option<u64>);

impl EscCode for PopPanel {
    const OPCODE: u16 = 0x61;
    fn args(&self) -> Vec<String> {
        encode_args![?self.0]
    }
}

/// Split the top panel of some section into two subsections.
/// * l_tag: The tag to identify the left/top subsection.
/// * r_tag: The tag to identify the right/bottom subsection.
/// * kind: The axis and position of the split in this panel.
/// * save: Which of the subsections the current contents of the panel should be saved to.
/// * rule: What rules should be applied to resizing the current contents to fit into the
/// subsection.
/// * split_tag: Which section to split (defaults to the active section).
/// * retain_offscreen_state: Whether new panels created by this split retain offscreen state
/// (defaults to true)
pub struct SplitPanel {
    pub l_tag: u64,
    pub r_tag: u64,
    pub kind: SplitKind,
    pub save: Option<SaveGrid>,
    pub rule: Option<ResizeRule>,
    pub split_tag: Option<u64>,
    pub retain_offscreen_state: Option<bool>,
}

impl SplitPanel {
    pub fn new(l_tag: u64,
               r_tag: u64,
               kind: SplitKind,
               save: Option<SaveGrid>,
               rule: Option<ResizeRule>,
               split_tag: Option<u64>,
               offscreen_state: Option<bool>) -> SplitPanel {
        SplitPanel {
            l_tag: l_tag,
            r_tag: r_tag,
            kind: kind,
            save: save,
            rule: rule,
            split_tag: split_tag,
            retain_offscreen_state: offscreen_state,
        }
    }
}

impl EscCode for SplitPanel {
    const OPCODE: u16 = 0x62;
    fn args(&self) -> Vec<String> {
        encode_args![self.l_tag, self.r_tag, self.kind, ?self.save, ?self.rule,
                     ?self.split_tag, ?self.retain_offscreen_state]
    }
}

/// Remove the split from a split panel.
/// * save: Which side of the split to save into the unsplit section.
/// * unsplit_tag: Which section's panel to unsplit 
pub struct UnsplitPanel {
    pub unsplit_tag: u64,
    pub save: SaveGrid,
}

impl UnsplitPanel {
    pub fn new(save: SaveGrid, unsplit_tag: u64) -> UnsplitPanel {
        UnsplitPanel {
            save: save,
            unsplit_tag: unsplit_tag,
        }
    }
}

impl EscCode for UnsplitPanel {
    const OPCODE: u16 = 0x63;
    fn args(&self) -> Vec<String> {
        encode_args![self.save, self.unsplit_tag]
    }
}

/// Adjust the split in a split panel.
/// * kind: The new axis and position for the split.
/// * rule: How to resize child panels.
/// * adjust_tag: Which section's panel to adjust the split of.
pub struct AdjustPanelSplit {
    pub kind: SplitKind,
    pub rule: ResizeRule,
    pub adjust_tag: u64,
}

impl AdjustPanelSplit {
    pub fn new(kind: SplitKind, rule: ResizeRule, adjust_tag: u64) -> AdjustPanelSplit {
        AdjustPanelSplit { kind: kind, rule: rule, adjust_tag: adjust_tag }
    }
}

impl EscCode for AdjustPanelSplit {
    const OPCODE: u16 = 0x64;
    fn args(&self) -> Vec<String> {
        encode_args![self.kind, self.rule, self.adjust_tag]
    }
}

/// Rotate the stack of a section down, putting the top panel on the bottom of the stack.
/// Argument: the tag of the section being rotated (defaults to active section).
pub struct RotateSectionDown(pub Option<u64>);

impl EscCode for RotateSectionDown {
    const OPCODE: u16 = 0x65;
    fn args(&self) -> Vec<String> {
        encode_args![?self.0]
    }
}

/// Rotate the stack of a section up, putting the bottom panel on the top of the stack.
/// Argument: the tag of the section being rotated (defaults to active section).
pub struct RotateSectionUp(pub Option<u64>);

impl EscCode for RotateSectionUp {
    const OPCODE: u16 = 0x66;
    fn args(&self) -> Vec<String> {
        encode_args![?self.0]
    }
}

/// Switch which section of the screen is currently active. The active section must have a grid
/// panel at the top of it.
/// Argument: The tag of the section to switch to.
pub struct SwitchActiveSection(pub u64);

impl EscCode for SwitchActiveSection {
    const OPCODE: u16 = 0x67;
    fn args(&self) -> Vec<String> {
        encode_args![self.0]
    }
}
