pub mod builtin;
pub mod builtins;
pub mod executable;

use builtin::Builtin;

use builtins::ChangeDirCommand;
use builtins::EchoComand;
use builtins::ExitComand;
use builtins::PwdComand;
use builtins::TypeComand;
use executable::Executable;

// Lis of all the known command.
pub const BUILT_IN_COMMANDS: [&str; 4] = ["exit", "echo", "type", "cd"];

pub enum Command {
    ExitCmd(ExitComand),
    EchoCmd(EchoComand),
    TypeCmd(TypeComand),
    PWDCmd(PwdComand),
    CDcmd(ChangeDirCommand),
    OtherCmd(Executable),
    CmdNotFound,
}

impl Command {
    // Parse the command into a varient of Command enum.
    pub fn parse(input: &str) -> Self {
        let mut input = Command::input_process(input).into_iter();
        // Check if the command is empty.
        let cmd = match input.next() {
            Some(comand) => comand,
            None => {
                return Command::CmdNotFound;
            }
        };

        let args: Vec<String> = input.collect();

        // Match the command and execute coresponding command
        match cmd.as_str() {
            "exit" => Command::ExitCmd(ExitComand { args }),
            "echo" => Command::EchoCmd(EchoComand {
                text: args.join(" "),
            }),
            "type" => Command::TypeCmd(TypeComand { args }),
            "pwd" => Command::PWDCmd(PwdComand),
            "cd" => Command::CDcmd(ChangeDirCommand { args }),
            other => Command::OtherCmd(Executable {
                name: other.to_string(),
                args,
            }),
        }
    }

    fn input_process(input: &str) -> Vec<String> {
        let input = input.chars();
        let mut previus_single_quot = false;
        let mut previus_dauble_quot = false;
        let mut token: Vec<char> = vec![];
        let mut output: Vec<String> = vec![];

        for c in input {
            match c {
                ' ' => {
                    if previus_single_quot || previus_dauble_quot {
                        token.push(c);
                    } else {
                        let final_token = token.iter().collect::<String>();
                        if !final_token.is_empty() {
                            output.push(final_token);
                            token.clear();
                        }
                    }
                }
                '"' => previus_dauble_quot = !previus_dauble_quot,
                '\'' => {
                    if previus_dauble_quot {
                        token.push(c);
                    } else {
                        previus_single_quot = !previus_single_quot;
                    }
                }
                other => {
                    token.push(other);
                }
            }
        }
        if !token.is_empty() {
            let final_token = token.iter().collect::<String>();
            output.push(final_token);
        }

        output
    }

    // Execute the coresponding command.
    pub fn execute(self) {
        match self {
            Command::ExitCmd(cmd) => cmd.execute(),
            Command::EchoCmd(cmd) => cmd.execute(),
            Command::TypeCmd(cmd) => cmd.execute(),
            Command::PWDCmd(cmd) => cmd.execute(),
            Command::CDcmd(cmd) => cmd.execute(),
            Command::OtherCmd(cmd) => cmd.execute(),
            //This line never going to call because it for handling empty user input
            Command::CmdNotFound => println!("command not found"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_process() {
        let input = "this is 'block name ' white";
        assert_eq!(
            Command::input_process(input),
            vec![
                "this".to_string(),
                "is".to_string(),
                "block name ".to_string(),
                "white".to_string()
            ]
        );
    }

    #[test]
    fn test_input_process_on_dauble_quote1() {
        let input = "\"hello\"world";
        assert_eq!(
            Command::input_process(input),
            vec!["helloworld".to_string()]
        );
    }

    #[test]
    fn test_input_process_on_dauble_quote2() {
        let input = "hello\"world\"";
        assert_eq!(
            Command::input_process(input),
            vec!["helloworld".to_string()]
        );
    }

    #[test]
    fn test_input_process_on_single_quote1() {
        let input = "\'hello\'world";
        assert_eq!(
            Command::input_process(input),
            vec!["helloworld".to_string()]
        );
    }

    #[test]
    fn test_input_process_on_single_quote2() {
        let input = "hello\'world\'";
        assert_eq!(
            Command::input_process(input),
            vec!["helloworld".to_string()]
        );
    }

    #[test]
    fn test_input_process_on_singlequote_inside_daublequote() {
        let input = "\"shell\"  \"example's\"  test\"\"script";
        assert_eq!(
            Command::input_process(input),
            vec![
                "shell".to_string(),
                "example's".to_string(),
                "testscript".to_string()
            ]
        );
    }
}
