# Persona UI Application

## Purpose

The persona project is used to have conversations with different AI agents. System context prompts are used to load
individual personas into an agent for a session, and they provide specific advice and guidance. In a terminal
configuration this works, but it's not easy to use and difficult to share with users that are less inclined to use a
terminal application. I still think communicating with the AI agents through the native terminal client is what's best,
band that's what this application will do, but make slightly simpler by removing some of the initial executoin from the
user. In the future improvements to the conversation UI could be made, but to simplify the initial prototype of this
application a TTY terminal component will be embedded into the application and used.

## Goals

A cross platform application that simplifies the process of having conversations with AI agents.

## Requirements

The project should adhere to the following requirements:

- be created in Rust
- use GPUI for the UI
- use the GPUI terminal component (gpui-terminal crate) for the terminal
- use the GPUI component library (gpui-component crate) for the UI components
- the persona list should display a list of personas that is read from a configuration file

## Preferences

These are some of my own personal preferences for developing the application:

- rust modules that are implemented as a directory with a mod.rs file are preferred over a single file
- smaller files that are created for a single purpose are preferred over larger files

## Design

```
                                      ┌──────────┬──────── Tabs for each conversation, clicking on
                                      │          │         option in list adds a tab if not already there
                ┌────┬──────────┬─────▼────┬─────▼────┬──────────────────────────────────┐
                │┌──┐│          │Persona 1 │Persona 2 │                                  │
 ┌──────────────►│  ││Personas  ├──────────┴──────────┴──────────────────────────────────┤
 │              │└──┘│          │ ┌──────────────────────────────────────────────────┬─┐ │
 │              │┌──┐├──────────┤ │                                                  │▲│ │
 │   ┌──────────►│  ││Persona 1 │ │                                                  │││ │
 │   │          │└──┘├──────────┤ │                                                  ├─┤ │
 │   │          │┌──┐│Persona 2 │ │                                                  │ │ │
 │   │   ┌──────►│  │├──────────┤ │                                                  │ │ │
 │   │   │      │└──┘│Persona 3 │ │                                                  │ │ │
 │   │   │      │    ├──────────┤ │                                                  │ │ │
 │   │   │      │    │          │ │                                                  │ │ │
 │   │  settings│    │          │ │                                                  │ │ │
 │   │  button  │    │          │ │                                                  │ │ │
 │   │          │    │          │ │                                                  │ │ │
 │  memory view │    │          │ │                                                  │ │ │
 │  button      │    │          │ │                                                  │ │ │
 │              │    │          │ │                                                  │ │ │
 │              │    │          │ │                                                  │ │ │
persona view    │    │          │ │                                                  ├─┤ │
button          │    │          │ │                                                  │││ │
                │    │          │ │          ▲                                       │▼│ │
                │    │          │ └──────────┼───────────────────────────────────────┴─┘ │
                └─▲──┴─────▲────┴───▲────────┼───────────────────────────────────────────┘
                  │        │        │        │
                  │        │        │        │
                  │        │        │        │
                  │        │        │       terminal component using gpui-terminal
                  │        │        │
                  │        │       main content area
                  │        │
                  │        │
                  │       persona selection list view
                  │
                  │
                 side navigation icon bar
```
