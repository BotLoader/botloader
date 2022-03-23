import type { Attachment } from "../discord/Attachment";
import type { ChannelMention } from "../discord/ChannelMention";
import type { Embed } from "../discord/Embed";
import type { IComponent } from "../discord/IComponent";
import type { IUser } from "./IUser";
import type { IUserMention } from "./UserMention";
import type { MessageActivity } from "../discord/MessageActivity";
import type { MessageApplication } from "../discord/MessageApplication";
import type { MessageFlags } from "../discord/MessageFlags";
import type { MessageReaction } from "../discord/MessageReaction";
import type { MessageReference } from "../discord/MessageReference";
import type { MessageType } from "../discord/MessageType";
import type { PartialMember } from "../discord/PartialMember";

export interface IMessage {
  activity: MessageActivity | null;
  application: MessageApplication | null;
  attachments: Array<Attachment>;
  author: IUser;
  channelId: string;
  content: string;
  components: Array<IComponent>;
  editedTimestamp: number | null;
  embeds: Array<Embed>;
  flags: MessageFlags | null;
  guildId: string | null;
  id: string;
  kind: MessageType;
  member: PartialMember | null;
  mentionChannels: Array<ChannelMention>;
  mentionEveryone: boolean;
  mentionRoles: Array<string>;
  mentions: Array<IUserMention>;
  pinned: boolean;
  reactions: Array<MessageReaction>;
  reference: MessageReference | null;
  referencedMessage: IMessage | null;
  timestamp: number;
  tts: boolean;
  webhookId: string | null;
}
