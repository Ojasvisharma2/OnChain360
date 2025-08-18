import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';
import type { IDL } from '@dfinity/candid';

export interface AuthResult { 'message' : [] | [string], 'success' : boolean }
export interface Comment {
  'id' : bigint,
  'post_id' : bigint,
  'content' : string,
  'timestamp' : bigint,
  'author_id' : bigint,
}
export interface DirectMessage {
  'id' : bigint,
  'receiver_id' : bigint,
  'content' : string,
  'sender_id' : bigint,
  'timestamp' : bigint,
}
export interface Notification {
  'id' : bigint,
  'read' : boolean,
  'user_id' : bigint,
  'message' : string,
  'timestamp' : bigint,
}
export interface Post {
  'id' : bigint,
  'content' : string,
  'likes' : BigUint64Array | bigint[],
  'timestamp' : bigint,
  'author_id' : bigint,
  'comments' : BigUint64Array | bigint[],
}
export type Result = { 'Ok' : string } |
  { 'Err' : AuthResult };
export type Result_1 = { 'Ok' : User } |
  { 'Err' : AuthResult };
export interface User {
  'id' : bigint,
  'bio' : [] | [string],
  'username' : string,
  'followers' : BigUint64Array | bigint[],
  'following' : BigUint64Array | bigint[],
}
export interface _SERVICE {
  'add_comment_api' : ActorMethod<[bigint, bigint, string], bigint>,
  'add_notification_api' : ActorMethod<[bigint, string], bigint>,
  'confirm_reset' : ActorMethod<[string, string, string], AuthResult>,
  'create_post_api' : ActorMethod<[bigint, string], bigint>,
  'follow' : ActorMethod<[bigint, bigint], boolean>,
  'get_comment' : ActorMethod<[bigint], [] | [Comment]>,
  'get_dms_api' : ActorMethod<[bigint], Array<DirectMessage>>,
  'get_notifications_api' : ActorMethod<[bigint], Array<Notification>>,
  'get_post_api' : ActorMethod<[bigint], [] | [Post]>,
  'get_post_likes' : ActorMethod<[bigint], BigUint64Array | bigint[]>,
  'get_user' : ActorMethod<[bigint], [] | [User]>,
  'like_post_api' : ActorMethod<[bigint, bigint], boolean>,
  'list_post_comments' : ActorMethod<[bigint], Array<Comment>>,
  'list_users' : ActorMethod<[], Array<User>>,
  'login_user' : ActorMethod<[string, string], AuthResult>,
  'request_reset' : ActorMethod<[string], Result>,
  'send_dm_api' : ActorMethod<[bigint, bigint, string], bigint>,
  'signup_user' : ActorMethod<
    [string, string, string, [] | [string]],
    Result_1
  >,
  'unfollow' : ActorMethod<[bigint, bigint], boolean>,
  'unlike_post_api' : ActorMethod<[bigint, bigint], boolean>,
}
export declare const idlFactory: IDL.InterfaceFactory;
export declare const init: (args: { IDL: typeof IDL }) => IDL.Type[];
