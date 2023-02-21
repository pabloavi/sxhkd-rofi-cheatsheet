// program that creates a cheatsheet in rofi for sxhkd keybindings

// Steps:
// 1. Read the sxhkdrc file
// 2. Parse the file and extract the keybindings, commands and descriptions
// 3. Create a cheatsheet in rofi with format: x.x | keybinding | description and executes the command on selection
// (x.x) represents a number to align them properly

// Internal process:
// 1. Read the file
// 2. Look for a line starting with super, alt, ctrl, or shift
// 3. If found, extract the keybinding (which is that line), command (following line) and description (previous line)

// import the necessary modules
use std::fs::File;
use std::io::Read;
use std::io::{BufRead, BufReader};

fn main() {
    // symbols dictionary
    let mut symbols = std::collections::HashMap::new();
    symbols.insert("super", " ");
    symbols.insert("alt", "Alt");
    symbols.insert("ctrl", "Ctrl");
    symbols.insert("shift", "וּ ");
    symbols.insert("Return", "↵ ");
    symbols.insert("@space", "␣ ");
    symbols.insert("Escape", "ESC ");

    let file = File::open("/home/pablo/.config/sxhkd/sxhkdrc").unwrap();
    let reader = BufReader::new(file);

    let mut keybindings = Vec::new();
    let mut commands = Vec::new();
    let mut descriptions = Vec::new();
    let mut line_numbers_of_keybindings = Vec::new();

    // FIRST LINES
    keybindings.push("bspwmrc".to_string());
    commands.push("alacritty -e nvim /home/pablo/.config/bspwm/bspwmrc".to_string());
    descriptions.push("Edit bspwmrc".to_string());
    line_numbers_of_keybindings.push(0);

    keybindings.push("sxhkdrc".to_string());
    commands.push("alacritty -e nvim /home/pablo/.config/sxhkd/sxhkdrc".to_string());
    descriptions.push("Edit sxhkdrc".to_string());
    line_numbers_of_keybindings.push(0);

    let mut lines = Vec::new();

    for line in reader.lines() {
        lines.push(line.unwrap());
    }

    for (i, line) in lines.iter().enumerate() {
        if line.starts_with("super")
            || line.starts_with("alt")
            || line.starts_with("ctrl")
            || line.starts_with("shift")
        {
            keybindings.push(line.to_string());

            commands.push(lines[i + 1].to_string());

            descriptions.push(lines[i - 1].to_string());

            line_numbers_of_keybindings.push(i);
        }
    }

    // remove all # from descriptions, first spaces and make first letter uppercase
    for (_i, description) in descriptions.iter_mut().enumerate() {
        description.retain(|c| c != '#');
        while description.starts_with(' ') {
            description.remove(0);
        }
        let mut chars = description.chars();
        if let Some(f) = chars.next() {
            description.replace_range(..1, &f.to_uppercase().to_string());
        }
    }

    // HEADERS (three consecutive # lines)
    let mut headers = Vec::new();
    let mut headers_line_numbers = Vec::new();

    for (i, line) in lines.iter().enumerate() {
        if line.starts_with("#") {
            if lines[i + 1].starts_with("#") {
                if lines[i + 2].starts_with("#") {
                    headers.push(lines[i + 1].to_string());
                    headers_line_numbers.push(i + 1);

                    for (_i, header) in headers.iter_mut().enumerate() {
                        header.retain(|c| c != '#');
                        while header.starts_with(' ') {
                            header.remove(0);
                        }
                        let mut chars = header.chars();
                        if let Some(f) = chars.next() {
                            header.replace_range(..1, &f.to_uppercase().to_string());
                        }
                    }
                }
            }
        }
    }

    // create a vector to store the formatted lines
    let mut formatted_lines = Vec::new();
    let mut j = 0;
    let mut k = 0;

    formatted_lines.push("0. EDIT CONFIG FILES".to_string());
    commands.insert(0, "".to_string());
    for (i, keybinding) in keybindings.iter().enumerate() {
        let mut formatted_line = String::new();
        k += 1;

        if headers_line_numbers[0] < line_numbers_of_keybindings[i] {
            if headers_line_numbers.len() > 1 {
                j += 1;
                k = 1;
                formatted_line.push_str(&j.to_string());
                formatted_line.push_str(". ");
                formatted_line.push_str(&(headers[0]));
                headers.remove(0);
                headers_line_numbers.remove(0);
                formatted_lines.push(formatted_line);
                // add an empty command to the commands vector in the same position as the added
                // formatted line
                commands.insert(i + j, "".to_string());
                formatted_line = String::new();
            }
        }

        formatted_line.push_str(&j.to_string());
        formatted_line.push_str(".");
        formatted_line.push_str(&k.to_string());
        formatted_line.push_str(" ");
        formatted_line.push_str(&keybinding);
        formatted_line.push_str(" ▶ ");
        formatted_line.push_str(&descriptions[i]);

        // FIXME: when replacing shift, it moves it to the end of the line
        // it should be replaced with the symbol and then the rest of the line should be the same
        // as the original
        // for (key, symbol) in symbols.iter() {
        //     formatted_line = formatted_line.replace(key, symbol);
        // }

        formatted_lines.push(formatted_line);
    }

    let mut formatted_lines_string = String::new();

    for formatted_line in formatted_lines.iter() {
        formatted_lines_string.push_str(&formatted_line);
        formatted_lines_string.push_str("\n");
    }

    // ROFI
    let mut rofi_command = String::new();

    rofi_command.push_str("rofi -dmenu -i -p 'sxhkd' -lines -font 'JetBrainsMono Nerd Font 14' ");
    rofi_command.push_str(&formatted_lines.len().to_string());

    let rofi_output = exec(&rofi_command, &formatted_lines_string);
    let rofi_output = rofi_output.trim_end();

    let mut selected_line = Vec::new();

    for formatted_line in formatted_lines.iter() {
        if formatted_line.eq(&rofi_output) {
            selected_line.push(formatted_line.to_string());
        }
    }

    // GET COMMAND COMPARING SELECTED LINE
    let mut selected_command = String::new();

    for (i, formatted_line) in formatted_lines.iter().enumerate() {
        if selected_line.is_empty() {
            break;
        }
        if formatted_line.eq(&selected_line[0]) {
            selected_command.push_str(&commands[i]);
        }
    }

    exec(&selected_command, "");
}

fn exec(command: &str, input: &str) -> String {
    use std::io::Write;
    use std::process::{Command, Stdio};

    let mut output = String::new();

    let mut child = Command::new("sh")
        .arg("-c")
        .arg(command)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("failed to execute process");

    // write the input to the command
    child
        .stdin
        .as_mut()
        .unwrap()
        .write_all(input.as_bytes())
        .expect("failed to write to stdin");

    // read the output from the command
    child
        .stdout
        .as_mut()
        .unwrap()
        .read_to_string(&mut output)
        .expect("failed to read stdout");

    // return the output
    output
}
