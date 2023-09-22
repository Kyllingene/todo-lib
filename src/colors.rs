use const_format::formatcp;

pub const RED: &str = "\x1b[38;5;1m";
pub const GREEN: &str = "\x1b[38;5;2m";
pub const YELLOW: &str = "\x1b[38;5;3m";
pub const DBLUE: &str = "\x1b[38;5;4m";
pub const PURPLE: &str = "\x1b[38;5;5m";
pub const LBLUE: &str = "\x1b[38;5;6m";
pub const GRAY: &str = "\x1b[38;5;7m";

pub const BOLD: &str = "\x1b[1m";
pub const ITALIC: &str = "\x1b[3m";
pub const UNDER: &str = "\x1b[4m";
pub const FADE: &str = "\x1b[2m";

pub(crate) const RESET: &str = "\x1b[0m";

pub const DEFAULT_STYLE: StyleScheme<'_> = StyleScheme {
    faded: formatcp!("{FADE}{GRAY}"),

    tick: "",
    priority: formatcp!("{BOLD}{LBLUE}"),
    completion: formatcp!("{UNDER}{PURPLE}"),
    creation: formatcp!("{UNDER}{YELLOW}"),

    description: "",
    context: formatcp!("{ITALIC}{GREEN}"),
    project: formatcp!("{ITALIC}{YELLOW}"),

    deadline: formatcp!("{BOLD}{RED}"),
    metadata: formatcp!("{ITALIC}{DBLUE}"),
};

#[derive(Clone, Copy, PartialEq, Eq, Debug, Default)]
pub struct StyleScheme<'a> {
    pub faded: &'a str,

    pub tick: &'a str,
    pub priority: &'a str,
    pub completion: &'a str,
    pub creation: &'a str,

    pub description: &'a str,
    pub context: &'a str,
    pub project: &'a str,

    pub deadline: &'a str,
    pub metadata: &'a str,
}

impl<'a> StyleScheme<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn faded(mut self, style: Option<&'a str>) -> Self {
        self.faded = style.unwrap_or("");
        self
    }

    pub fn tick(mut self, style: Option<&'a str>) -> Self {
        self.tick = style.unwrap_or("");
        self
    }

    pub fn priority(mut self, style: Option<&'a str>) -> Self {
        self.priority = style.unwrap_or("");
        self
    }

    pub fn completion(mut self, style: Option<&'a str>) -> Self {
        self.completion = style.unwrap_or("");
        self
    }

    pub fn creation(mut self, style: Option<&'a str>) -> Self {
        self.creation = style.unwrap_or("");
        self
    }

    pub fn description(mut self, style: Option<&'a str>) -> Self {
        self.description = style.unwrap_or("");
        self
    }

    pub fn context(mut self, style: Option<&'a str>) -> Self {
        self.context = style.unwrap_or("");
        self
    }

    pub fn project(mut self, style: Option<&'a str>) -> Self {
        self.project = style.unwrap_or("");
        self
    }

    pub fn deadline(mut self, style: Option<&'a str>) -> Self {
        self.deadline = style.unwrap_or("");
        self
    }

    pub fn metadata(mut self, style: Option<&'a str>) -> Self {
        self.metadata = style.unwrap_or("");
        self
    }

    pub fn get_colors(&self, fade: bool) -> (&'static str, Self) {
        if fade {
            ("", Self::new().faded(Some(self.faded)))
        } else {
            (RESET, self.faded(None))
        }
    }
}
