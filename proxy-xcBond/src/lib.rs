#![no_std]

use gstd::{msg, ActorId, MessageId};
use io::{Action, Event};

static mut SESSION: Option<Session> = None;

struct Session{
    bonding_program_id: Vec<ActorId>, // smart program del bonding variable y muliple para muchos proyectos
    msg_id_to_actor_id: (MessageId, ActorId), //el ide del mensaje k esta mandando el usuario y el actorID del usuario
}


#[no_mangle]
extern "C" fn init() {
   let target_program_id: ActorId = msg::load().expect("Failed to load target program id");
   unsafe { 
    SESSION = Some(Session{
        bonding_program_id = Vec::new(),
        msg_id_to_actor_id: (MessageId::zero(), ActorId::zero()),
    })
   }
}

#[no_mangle]
extern  "C" fn handle(){
    let action: Action = msg::load().expect("Unable to decode action");
    let session = unsafe { SESSION.as_mut().expect("Session not initialized") };
    let msg_id = msg::send(session.target_program_id, action, 0).expect("error in sending message");
    session.msg_id_to_actor_id = (msg_id, msg::source());
    msg::reply(Event::MessageSent, 0).expect("Unable to send reply");
}

#[no_mangle]
extern "C" fn handle_reply(){
  let reply_msg_id = msg::reply_to().expect("Unable to get reply message id");
  let session = unsafe { SESSION.as_mut().expect("Session not initialized") };
  let (msg_id, actor) = session.msg_id_to_actor_id;
  if reply_msg_id == msg_id {
    let reply: Event = msg::load().expect("Unable to decode reply");
    msg::send(actor, reply, 0).expect("Unable to send reply");
  }
}

