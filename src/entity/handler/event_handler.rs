-use twilight::gateway::Event;
use twilight::model::gateway::payload::*;
use twilight::model::gateway::event::shard::*;

use std::error::Error;
use std::sync::Arc;
use std::fmt;

use async_trait::async_trait;

use crate::entity::client::Client;

#[async_trait]
pub trait EventHandler: Sync {

    async fn handle_event(
        &self,
        event: (u64, Event),
        client: Arc<Client>,
    ) -> Result<(), Box<dyn Error + Send + Sync>> {

        match event {
            (_, Event::BanAdd(ban)) => self.ban_add(client.clone(), ban).await,
            (_, Event::BanRemove(ban)) => self.ban_remove(client.clone(), ban).await,
            (_, Event::ChannelCreate(channel)) => self.channel_create(client.clone(), channel).await,
            (_, Event::ChannelDelete(channel)) => self.channel_delete(client.clone(), channel).await,
            (_, Event::ChannelPinsUpdate(channel)) => self.channel_pins_update(client.clone(), channel).await,
            (_, Event::ChannelUpdate(channel)) => self.channel_update(client.clone(), channel).await,
            (_, Event::GatewayHeartbeat(beat)) => self.gateway_heartbeat(client.clone(), beat).await,
            (_, Event::GatewayHeartbeatAck) => self.gateway_heartbeat_ack(client.clone()).await,
            (_, Event::GatewayHello(hi)) => self.gateway_hello(client.clone(), hi).await,
            (_, Event::GatewayInvalidateSession(invalidate)) => self.gateway_invalidate_session(client.clone(), invalidate).await,
            (_, Event::GatewayReconnect) => self.gateway_reconnect(client.clone()).await,
            (_, Event::GuildCreate(create)) => self.guild_create(client.clone(), create).await,
            (_, Event::GuildDelete(delete)) => self.guild_delete(client.clone(), delete).await,
            (_, Event::GuildEmojisUpdate(update)) => self.guild_emojis_update(client.clone(), update).await,
            (_, Event::GuildIntegrationsUpdate(update)) => self.guild_integrations_update(client.clone(), update).await,
            (_, Event::GuildUpdate(update)) => self.guild_update(client.clone(), update).await,
            (_, Event::InviteCreate(create)) => self.invite_create(client.clone(), create).await,
            (_, Event::InviteDelete(delete)) => self.invite_delete(client.clone(), delete).await,
            (_, Event::MemberAdd(add)) => self.member_add(client.clone(), add).await,
            (_, Event::MemberRemove(remove)) => self.member_remove(client.clone(), remove).await,
            (_, Event::MemberUpdate(update)) => self.member_update(client.clone(), update).await,
            (_, Event::MemberChunk(chunk)) => self.member_chunk(client.clone(), chunk).await,
            (_, Event::MessageCreate(create)) => self.message_create(client.clone(), create).await,
            (_, Event::MessageDelete(delete)) => self.message_delete(client.clone(), delete).await,
            (_, Event::MessageDeleteBulk(delete_bulk)) => self.message_delete_bulk(client.clone(), delete_bulk).await,
            (_, Event::MessageUpdate(update)) => self.message_update(client.clone(), update).await,
            (_, Event::PresenceUpdate(update)) => self.presence_update(client.clone(), update).await,
            (_, Event::PresencesReplace) => self.presences_replace(client.clone()).await,
            (_, Event::ReactionAdd(reaction)) => self.reaction_add(client.clone(), reaction).await,
            (_, Event::ReactionRemove(remove)) => self.reaction_remove(client.clone(), remove).await,
            (_, Event::ReactionRemoveAll(remove_all)) => self.reaction_remove_all(client.clone(), remove_all).await,
            (_, Event::ReactionRemoveEmoji(remove_emoji)) => self.reaction_remove_emoji(client.clone(), remove_emoji).await,
            (_, Event::Ready(ready)) => self.ready(client.clone(), ready).await,
            (_, Event::Resumed) => self.resumed(client.clone()).await,
            (_, Event::RoleCreate(create)) => self.role_create(client.clone(), create).await,
            (_, Event::RoleDelete(delete)) => self.role_delete(client.clone(), delete).await,
            (_, Event::RoleUpdate(update)) => self.role_update(client.clone(), update).await,
            (_, Event::ShardConnected(connected)) => self.shard_connected(client.clone(), connected).await,
            (_, Event::ShardConnecting(connecting)) => self.shard_connecting(client.clone(), connecting).await,
            (_, Event::ShardDisconnected(disconnected)) => self.shard_disconnected(client.clone(), disconnected).await,
            (_, Event::ShardIdentifying(identifying)) => self.shard_identifying(client.clone(), identifying).await,
            (_, Event::ShardReconnecting(reconnecting)) => self.shard_reconnecting(client.clone(), reconnecting).await,
            (_, Event::ShardPayload(payload)) => self.shard_payload(client.clone(), payload).await,
            (_, Event::ShardResuming(resuming)) => self.shard_resuming(client.clone(), resuming).await,
            (_, Event::TypingStart(start)) => self.typing_start(client.clone(), start).await,
            (_, Event::UnavailableGuild(guild_uv)) => self.unavailable_guild(client.clone(), guild_uv).await,
            (_, Event::UserUpdate(update)) => self.user_update(client.clone(), update).await,
            (_, Event::VoiceServerUpdate(update)) => self.voice_server_update(client.clone(), update).await,
            (_, Event::VoiceStateUpdate(update)) => self.voice_state_update(client.clone(), update).await,
            (_, Event::WebhooksUpdate(update)) => self.webhooks_update(client.clone(), update).await
        }.expect("Event handler failed!");

        Ok(())

    }

    async fn ban_add(&self, _: Arc<Client>, _: BanAdd) -> Result<(), Box<dyn Error + Send + Sync>> { Ok(()) }
    async fn ban_remove(&self, _: Arc<Client>, _: BanRemove) -> Result<(), Box<dyn Error + Send + Sync>> { Ok(()) }
    async fn channel_create(&self, _: Arc<Client>, _: ChannelCreate) -> Result<(), Box<dyn Error + Send + Sync>> { Ok(()) }
    async fn channel_delete(&self, _: Arc<Client>, _: ChannelDelete) -> Result<(), Box<dyn Error + Send + Sync>> { Ok(()) }
    async fn channel_pins_update(&self, _: Arc<Client>, _: ChannelPinsUpdate) -> Result<(), Box<dyn Error + Send + Sync>> { Ok(()) }
    async fn channel_update(&self, _: Arc<Client>, _: ChannelUpdate) -> Result<(), Box<dyn Error + Send + Sync>> { Ok(()) }
    async fn gateway_heartbeat(&self, _: Arc<Client>, _: u64) -> Result<(), Box<dyn Error + Send + Sync>> { Ok(()) }
    async fn gateway_heartbeat_ack(&self, _: Arc<Client>) -> Result<(), Box<dyn Error + Send + Sync>> { Ok(()) }
    async fn gateway_hello(&self, _: Arc<Client>, _: u64) -> Result<(), Box<dyn Error + Send + Sync>> { Ok(()) }
    async fn gateway_invalidate_session(&self, _: Arc<Client>, _: bool) -> Result<(), Box<dyn Error + Send + Sync>> { Ok(()) }
    async fn gateway_reconnect(&self, _: Arc<Client>) -> Result<(), Box<dyn Error + Send + Sync>> { Ok(()) }
    async fn guild_create(&self, _: Arc<Client>, _: Box<GuildCreate>) -> Result<(), Box<dyn Error + Send + Sync>> { Ok(()) }
    async fn guild_delete(&self, _: Arc<Client>, _: Box<GuildDelete>) -> Result<(), Box<dyn Error + Send + Sync>> { Ok(()) }
    async fn guild_emojis_update(&self, _: Arc<Client>, _: GuildEmojisUpdate) -> Result<(), Box<dyn Error + Send + Sync>> { Ok(()) }
    async fn guild_integrations_update(&self, _: Arc<Client>, _: GuildIntegrationsUpdate) -> Result<(), Box<dyn Error + Send + Sync>> { Ok(()) }
    async fn guild_update(&self, _: Arc<Client>, _: Box<GuildUpdate>) -> Result<(), Box<dyn Error + Send + Sync>> { Ok(()) }
    async fn invite_create(&self, _: Arc<Client>, _: Box<InviteCreate>) -> Result<(), Box<dyn Error + Send + Sync>> { Ok(()) }
    async fn invite_delete(&self, _: Arc<Client>, _: InviteDelete) -> Result<(), Box<dyn Error + Send + Sync>> { Ok(()) }
    async fn member_add(&self, _: Arc<Client>, _: Box<MemberAdd>) -> Result<(), Box<dyn Error + Send + Sync>> { Ok(()) }
    async fn member_remove(&self, _: Arc<Client>, _: MemberRemove) -> Result<(), Box<dyn Error + Send + Sync>> { Ok(()) }
    async fn member_update(&self, _: Arc<Client>, _: Box<MemberUpdate>) -> Result<(), Box<dyn Error + Send + Sync>> { Ok(()) }
    async fn member_chunk(&self, _: Arc<Client>, _: MemberChunk) -> Result<(), Box<dyn Error + Send + Sync>> { Ok(()) }
    async fn message_create(&self, _: Arc<Client>, _: Box<MessageCreate>) -> Result<(), Box<dyn Error + Send + Sync>> { Ok(()) }
    async fn message_delete(&self, _: Arc<Client>, _: MessageDelete) -> Result<(), Box<dyn Error + Send + Sync>> { Ok(()) }
    async fn message_delete_bulk(&self, _: Arc<Client>, _: MessageDeleteBulk) -> Result<(), Box<dyn Error + Send + Sync>> { Ok(()) }
    async fn message_update(&self, _: Arc<Client>, _: Box<MessageUpdate>) -> Result<(), Box<dyn Error + Send + Sync>> { Ok(()) }
    async fn presence_update(&self, _: Arc<Client>, _: Box<PresenceUpdate>) -> Result<(), Box<dyn Error + Send + Sync>> { Ok(()) }
    async fn presences_replace(&self, _: Arc<Client>) -> Result<(), Box<dyn Error + Send + Sync>> { Ok(()) }
    async fn reaction_add(&self, _: Arc<Client>, _: Box<ReactionAdd>) -> Result<(), Box<dyn Error + Send + Sync>> { Ok(()) }
    async fn reaction_remove(&self, _: Arc<Client>, _: Box<ReactionRemove>) -> Result<(), Box<dyn Error + Send + Sync>> { Ok(()) }
    async fn reaction_remove_all(&self, _: Arc<Client>, _: ReactionRemoveAll) -> Result<(), Box<dyn Error + Send + Sync>> { Ok(()) }
    async fn reaction_remove_emoji(&self, _: Arc<Client>, _: ReactionRemoveEmoji) -> Result<(), Box<dyn Error + Send + Sync>> { Ok(()) }
    async fn ready(&self, _: Arc<Client>, _: Box<Ready>) -> Result<(), Box<dyn Error + Send + Sync>> { Ok(()) }
    async fn resumed(&self, _: Arc<Client>) -> Result<(), Box<dyn Error + Send + Sync>> { Ok(()) }
    async fn role_create(&self, _: Arc<Client>, _: RoleCreate) -> Result<(), Box<dyn Error + Send + Sync>> { Ok(()) }
    async fn role_delete(&self, _: Arc<Client>, _: RoleDelete) -> Result<(), Box<dyn Error + Send + Sync>> { Ok(()) }
    async fn role_update(&self, _: Arc<Client>, _: RoleUpdate) -> Result<(), Box<dyn Error + Send + Sync>> { Ok(()) }
    async fn shard_connected(&self, _: Arc<Client>, _: Connected) -> Result<(), Box<dyn Error + Send + Sync>> { Ok(()) }
    async fn shard_connecting(&self, _: Arc<Client>, _: Connecting) -> Result<(), Box<dyn Error + Send + Sync>> { Ok(()) }
    async fn shard_disconnected(&self, _: Arc<Client>, _: Disconnected) -> Result<(), Box<dyn Error + Send + Sync>> { Ok(()) }
    async fn shard_identifying(&self, _: Arc<Client>, _: Identifying) -> Result<(), Box<dyn Error + Send + Sync>> { Ok(()) }
    async fn shard_reconnecting(&self, _: Arc<Client>, _: Reconnecting) -> Result<(), Box<dyn Error + Send + Sync>> { Ok(()) }
    async fn shard_payload(&self, _: Arc<Client>, _: Payload) -> Result<(), Box<dyn Error + Send + Sync>> { Ok(()) }
    async fn shard_resuming(&self, _: Arc<Client>, _: Resuming) -> Result<(), Box<dyn Error + Send + Sync>> { Ok(()) }
    async fn typing_start(&self, _: Arc<Client>, _: Box<TypingStart>) -> Result<(), Box<dyn Error + Send + Sync>> { Ok(()) }
    async fn unavailable_guild(&self, _: Arc<Client>, _: UnavailableGuild) -> Result<(), Box<dyn Error + Send + Sync>> { Ok(()) }
    async fn user_update(&self, _: Arc<Client>, _: UserUpdate) -> Result<(), Box<dyn Error + Send + Sync>> { Ok(()) }
    async fn voice_server_update(&self, _: Arc<Client>, _: VoiceServerUpdate) -> Result<(), Box<dyn Error + Send + Sync>> { Ok(()) }
    async fn voice_state_update(&self, _: Arc<Client>, _: Box<VoiceStateUpdate>) -> Result<(), Box<dyn Error + Send + Sync>> { Ok(()) }
    async fn webhooks_update(&self, _: Arc<Client>, _: WebhooksUpdate) -> Result<(), Box<dyn Error + Send + Sync>> { Ok(()) }

}

impl fmt::Debug for dyn EventHandler + Send + Sync {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("EventHandler")
         .field("up", &true)
         .finish()
    }
}