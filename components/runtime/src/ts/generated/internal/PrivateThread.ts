import type { IPermissionOverwrite } from "../discord/IPermissionOverwrite";
import type { ISelfThreadMember } from "./ISelfThreadMember";
import type { ThreadMetadata } from "../discord/ThreadMetadata";

export interface IPrivateThread {
  defaultAutoArchiveDurationMinutes: number | null;
  id: string;
  invitable: boolean | null;
  kind: "PrivateThread";
  member: ISelfThreadMember | null;
  memberCount: number;
  messageCount: number;
  name: string;
  ownerId: string | null;
  parentId: string | null;
  permissionOverwrites: Array<IPermissionOverwrite>;
  rateLimitPerUser: number | null;
  threadMetadata: ThreadMetadata;
}
