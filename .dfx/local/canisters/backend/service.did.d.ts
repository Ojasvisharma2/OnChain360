import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';
import type { IDL } from '@dfinity/candid';

export interface _SERVICE {
  'add_post' : ActorMethod<[string], undefined>,
  'add_user' : ActorMethod<[string], undefined>,
  'get_posts' : ActorMethod<[], string>,
  'get_users' : ActorMethod<[], string>,
  'greet' : ActorMethod<[string], string>,
}
export declare const idlFactory: IDL.InterfaceFactory;
export declare const init: (args: { IDL: typeof IDL }) => IDL.Type[];
