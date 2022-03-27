# Clockify TUI
Clockify TUI written in Rust

## Installation
`cargo install clockify`

## Usage
`clockify -a <API_KEY>`

Note: The API Key only needs to be set the first time the program is executed.

## Keyboard Shortcuts

| Shortcut | Action |
|----------|--------|
| t | Go to Time Entry selection screen. |
| w | Go to Workspace selection screen. |
| p | Go to Project selection screen. |
| g | Go to Tag selection screen. |
| y | Go to Task selection screen. |
| h | Go to Home screen. |
| CTRL + q | Quit the application. |
| i | Enter Edit mode |
| / | Enter Search mode |
| ESC | Exit Edit or Search mode |
| j | Navigate down |
| k | Navigate up |
| c | Clear selections |
| s | Start timer (from Home screen)
| e | End timer (from Home screen)

## Endpoints
- [ ] Client
- [X] Project
- [X] Tag
- [X] Task
- [X] Time Entry
- [ ] Expense
- [ ] Invoice
- [X] User
- [ ] Group
- [X] Workspace
- [ ] Custom Fields
- [ ] Approvals
