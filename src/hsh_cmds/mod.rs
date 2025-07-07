use std::{str, fmt, result};

#[derive(Debug)]
/// Error type for HshCmds.
/// Currently used for unknown commands and flag errors.
///
/// # Example
/// ```rust
/// use hsh::hsh_cmds::{HshCmdError, HshCmds};
///
/// eprintln!("{}", HshCmdError::CmdErr("mcdir".to_string()));
/// eprintln!("{}", HshCmdError::FlagErr(HshCmds::Ls, "-f".to_string()));
/// // Output: Unknown command 'mcdir'.
/// // Output: Unknown flag '-f' for `ls`.
/// ```
pub enum HshCmdError {
    CmdErr(String),
    FlagErr(HshCmds, String),
}

impl fmt::Display for HshCmdError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::CmdErr(s) => write!(f, "Unknown command '{}'.", s),
            Self::FlagErr(cmd, s) => write!(f, "Unknown flag '{}' for `{}`.", s, cmd),
        }
    }
}

#[derive(Debug)]
/// Enum containing all of the built-in commands for the Hsh Shell.
pub enum HshCmds {
    Ls,
    Cd,
    Clear,
}

impl fmt::Display for HshCmds {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Ls => write!(f, "ls"),
            Self::Cd => write!(f, "cd"),
            Self::Clear => write!(f, "clear"),
        }
    }
}

impl str::FromStr for HshCmds {
    type Err = HshCmdError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "ls" => Ok(Self::Ls),
            "cd" => Ok(Self::Cd),
            "clear" => Ok(Self::Clear),
            e => Err(HshCmdError::CmdErr(e.to_string()))
        }
    }
}

impl HshCmds {
    pub fn from_input(input: Self, flags: Flags) -> result::Result<(Self, self::Command), HshCmdError> {
        match input {
            Self::Ls => {
                let command_struct = input.build_command(flags)?;
                // command_struct.execute_ls();
                Ok((input, command_struct))
            },
            Self::Cd => {
                let command_struct = input.build_command(flags)?;
                Ok((input, command_struct))
            },
            Self::Clear => {
                let command_struct = input.build_command(flags)?;
                Ok((input, command_struct))
            },
        }
    }

    fn build_command(&self, flags: Flags) -> result::Result<self::Command, HshCmdError> {
        match self {
            Self::Ls => {
                let mut flags_passed: Flags = Flags::default();
                for flag in flags {
                    if let Some(f) = self.valid_flags() {
                        if f.contains(&flag) {
                            flags_passed.push(flag);
                        } else {
                            return Err(HshCmdError::FlagErr(HshCmds::Ls, format!("{}", flag)));
                        }
                    }
                }
                Ok(self::Command::new(HshCmds::Ls, Some(flags_passed)))
            },
            Self::Cd => {
                if !flags.is_empty() {
                    return Err(HshCmdError::FlagErr(HshCmds::Cd, format!("{}", flags)));
                }
                Ok(self::Command::new(HshCmds::Cd, None))
            },
            Self::Clear => {
                if !flags.is_empty() {
                    return Err(HshCmdError::FlagErr(HshCmds::Clear, format!("{}", flags)));
                }
                Ok(self::Command::new(HshCmds::Clear, None))
            },
        }
    }

    fn valid_flags(&self) -> Option<Flags> {
        let mut flags = Flags::default();
        match self {
            Self::Ls => {
                flags.push(Flag::new('l'));
                flags.push(Flag::new('a'));
            },
            Self::Cd => {return None;},
            Self::Clear => {return None;},
        }
        Some(flags)
    }
}

pub struct Command {
    pub cmd: HshCmds,
    opts: Option<Flags>,
}

impl Command {
    fn new(cmd: HshCmds, opts: Option<Flags>) -> Self {
        Self {
            cmd,
            opts,
        }
    }

    // pub fn execute_ls(&self) {
    //     if let Some(arg) = self.opts {
    //         match arg {
    //         "-l" => println!("yaya contains '-l'"),
    //         _ => println!("invalid option")
    //     }}
    //     let mut list_dir = fs::read_dir(dir)?
    //         .map(|e| e.map(|res| res.file_name()))
    //         .collect::<Result<Vec<_>, io::Error>>()?;
    //
    //     list_dir.retain(|os_string| {
    //         if let Some(s) = os_string.to_str() {
    //             !s.starts_with(".")
    //         } else {
    //             false
    //         }
    //     });
    //
    //     list_dir.sort();
    //     let mut output_string = String::new();
    //     for (index, item) in list_dir.iter().enumerate() {
    //         if let Some(s) = item.to_str() {
    //             output_string.push_str(s);
    //         }
    //         if index < list_dir.len() - 1 {
    //             output_string.push_str("    ");
    //         }
    //     }
    //
    //     println!("{}", output_string);
    //
    //     Ok(())
    // }

    pub fn execute_cd(&self) {}
}

#[derive(Debug, PartialEq, Default, Clone)]
/// A struct representing all of the flags for a command.
///
/// # Example
/// ```rust
/// use hsh::hsh_cmds::{Flags, Flag};
///
/// let f1 = Flag::new('a');
/// let f2 = Flag::new('b');
/// let f3 = Flag::new('c');
/// let f4 = Flag::new('d');
/// let f5 = Flag::new('e');
///
/// let flags = Flags(vec![f1, f2, f3, f4, f5]);
///
/// let f6 = Flag::new('e');
/// assert!(flags.contains(&f6));
/// assert_eq!(format!("{}", flags), format!("-abcde"))
/// ```
pub struct Flags(pub Vec<Flag>);

impl Iterator for Flags {
    type Item = Flag;
    fn next(&mut self) -> Option<Self::Item> {
        let cloned_self = self.0.clone();
        cloned_self.into_iter().next()
    }
}

impl fmt::Display for Flags {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut flag_str = String::new();
        for flag in &self.0 {
            flag_str.push(flag.0);
        }
        write!(f, "-{}", flag_str)
    }
}

impl Flags {
    pub fn push(&mut self, flag: Flag) -> &mut Self {
        self.0.push(flag);
        self
    }
    pub fn contains(&self, flag: &Flag) -> bool {
        self.0.contains(flag)
    }
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
/// A struct that represents a command's option flag.
///
/// For example, take this command: `ls -l`
/// the `-l` is represented by a `Flag` as `Flag('l')`.
/// 
/// Implements `fmt::Display` to display `Flag('l')` as `l`.
/// Implements `std::str::FromStr` to construct a `Flag` from a string.
///
/// # Example
/// ```rust
/// use hsh::hsh_cmds::Flag;
/// let flag_l = "l".parse::<Flag>().unwrap();
/// assert_eq!(flag_l, Flag('l'));
/// ```
pub struct Flag(pub char);

impl Flag {
    /// Creates a new Flag using the given `char`.
    pub fn new(flag: char) -> Self {
        Flag(flag)
    }
}

impl fmt::Display for Flag {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl str::FromStr for Flag {
    type Err = FlagErr;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() > 2 || s.is_empty() {
            return Err(FlagErr(s.to_string()));
        }

        if s.starts_with('-') {
            let s = s.strip_prefix('-').expect("Verified that string starts with '-'.");
            Ok(Self(s.chars().next().unwrap()))
        } else {
            Ok(Self(s.chars().next().unwrap()))
        }
    }
}

#[derive(Debug)]
/// An error for invalid flags, right now this
/// error is only thrown if `.parse::<Flag>()` fails.
pub struct FlagErr(String);

impl fmt::Display for FlagErr {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Invalid flag: {}", self.0)
    }
}
