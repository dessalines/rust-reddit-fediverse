use crate::{
  activities::send::generate_activity_id,
  activity_queue::send_activity_single_dest,
  ActorType,
  ApubObjectType,
  ToApub,
};
use activitystreams::{
  activity::{
    kind::{CreateType, DeleteType, UndoType, UpdateType},
    Create,
    Delete,
    Undo,
    Update,
  },
  prelude::*,
};
use lemmy_db::{private_message::PrivateMessage, user::User_, Crud};
use lemmy_structs::blocking;
use lemmy_utils::LemmyError;
use lemmy_websocket::LemmyContext;

#[async_trait::async_trait(?Send)]
impl ApubObjectType for PrivateMessage {
  /// Send out information about a newly created private message
  async fn send_create(&self, creator: &User_, context: &LemmyContext) -> Result<(), LemmyError> {
    let note = self.to_apub(context.pool()).await?;

    let recipient_id = self.recipient_id;
    let recipient = blocking(context.pool(), move |conn| User_::read(conn, recipient_id)).await??;

    let mut create = Create::new(creator.actor_id.to_owned(), note.into_any_base()?);
    let to = recipient.get_inbox_url()?;
    create
      .set_context(activitystreams::context())
      .set_id(generate_activity_id(CreateType::Create)?)
      .set_to(to.clone());

    send_activity_single_dest(create, creator, to, context).await?;
    Ok(())
  }

  /// Send out information about an edited post, to the followers of the community.
  async fn send_update(&self, creator: &User_, context: &LemmyContext) -> Result<(), LemmyError> {
    let note = self.to_apub(context.pool()).await?;

    let recipient_id = self.recipient_id;
    let recipient = blocking(context.pool(), move |conn| User_::read(conn, recipient_id)).await??;

    let mut update = Update::new(creator.actor_id.to_owned(), note.into_any_base()?);
    let to = recipient.get_inbox_url()?;
    update
      .set_context(activitystreams::context())
      .set_id(generate_activity_id(UpdateType::Update)?)
      .set_to(to.clone());

    send_activity_single_dest(update, creator, to, context).await?;
    Ok(())
  }

  async fn send_delete(&self, creator: &User_, context: &LemmyContext) -> Result<(), LemmyError> {
    let note = self.to_apub(context.pool()).await?;

    let recipient_id = self.recipient_id;
    let recipient = blocking(context.pool(), move |conn| User_::read(conn, recipient_id)).await??;

    let mut delete = Delete::new(creator.actor_id.to_owned(), note.into_any_base()?);
    let to = recipient.get_inbox_url()?;
    delete
      .set_context(activitystreams::context())
      .set_id(generate_activity_id(DeleteType::Delete)?)
      .set_to(to.clone());

    send_activity_single_dest(delete, creator, to, context).await?;
    Ok(())
  }

  async fn send_undo_delete(
    &self,
    creator: &User_,
    context: &LemmyContext,
  ) -> Result<(), LemmyError> {
    let note = self.to_apub(context.pool()).await?;

    let recipient_id = self.recipient_id;
    let recipient = blocking(context.pool(), move |conn| User_::read(conn, recipient_id)).await??;

    let mut delete = Delete::new(creator.actor_id.to_owned(), note.into_any_base()?);
    let to = recipient.get_inbox_url()?;
    delete
      .set_context(activitystreams::context())
      .set_id(generate_activity_id(DeleteType::Delete)?)
      .set_to(to.clone());

    // Undo that fake activity
    let mut undo = Undo::new(creator.actor_id.to_owned(), delete.into_any_base()?);
    undo
      .set_context(activitystreams::context())
      .set_id(generate_activity_id(UndoType::Undo)?)
      .set_to(to.clone());

    send_activity_single_dest(undo, creator, to, context).await?;
    Ok(())
  }

  async fn send_remove(&self, _mod_: &User_, _context: &LemmyContext) -> Result<(), LemmyError> {
    unimplemented!()
  }

  async fn send_undo_remove(
    &self,
    _mod_: &User_,
    _context: &LemmyContext,
  ) -> Result<(), LemmyError> {
    unimplemented!()
  }
}