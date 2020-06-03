use ::std::time;
use ::std::thread::park_timeout;
use crate::messages::Instruction;

pub enum Event {
    PathChange,
    PathError,
    FileDeleted,
    AppExit,
}

use std::sync::mpsc::{SyncSender, Receiver};

pub fn handle_events (event_receiver: Receiver<Event>, instruction_sender: SyncSender<Instruction>) {
    loop {
        let event = event_receiver.recv().expect("failed to receive event on channel");
        match event {
            Event::PathChange => {
                let _ = instruction_sender.send(Instruction::SetFrameAroundCurrentPath);
                let _ = instruction_sender.send(Instruction::Render);
                park_timeout(time::Duration::from_millis(250));
                let _ = instruction_sender.send(Instruction::RemoveFrameAroundCurrentPath);
                let _ = instruction_sender.send(Instruction::Render);
            }
            Event::PathError => {
                let _ = instruction_sender.send(Instruction::SetPathToRed);
                let _ = instruction_sender.send(Instruction::SetFrameAroundCurrentPath);
                let _ = instruction_sender.send(Instruction::Render);
                park_timeout(time::Duration::from_millis(250));
                let _ = instruction_sender.send(Instruction::ResetCurrentPathColor);
                let _ = instruction_sender.send(Instruction::RemoveFrameAroundCurrentPath);
                let _ = instruction_sender.send(Instruction::Render);
            }
            Event::FileDeleted => {
                let _ = instruction_sender.send(Instruction::SetFrameAroundSpaceFreed);
                let _ = instruction_sender.send(Instruction::Render);
                park_timeout(time::Duration::from_millis(250));
                let _ = instruction_sender.send(Instruction::RemoveFrameAroundSpaceFreed);
                let _ = instruction_sender.send(Instruction::Render);
            }
            Event::AppExit => {
                break;
            }
        }
    }
}