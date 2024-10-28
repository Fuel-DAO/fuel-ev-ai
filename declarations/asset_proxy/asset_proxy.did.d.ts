import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';
import type { IDL } from '@dfinity/candid';

export interface _SERVICE {
  'approve_files' : ActorMethod<
    [{ 'files' : Array<string>, 'asset_canister' : Principal }],
    { 'Ok' : boolean } |
      { 'Err' : string }
  >,
  'get_provision_canister' : ActorMethod<[], Principal>,
  'get_temp_asset_canister' : ActorMethod<[], Principal>,
  'prune' : ActorMethod<
    [Array<string>],
    { 'Ok' : boolean } |
      { 'Err' : string }
  >,
  'reject_files' : ActorMethod<
    [Array<string>],
    { 'Ok' : boolean } |
      { 'Err' : string }
  >,
  'set_provision_canister' : ActorMethod<
    [Principal],
    { 'Ok' : boolean } |
      { 'Err' : string }
  >,
  'set_temp_asset_canister' : ActorMethod<
    [Principal],
    { 'Ok' : boolean } |
      { 'Err' : string }
  >,
  'store' : ActorMethod<
    [
      {
        'key' : string,
        'content' : Uint8Array | number[],
        'sha256' : [] | [Uint8Array | number[]],
        'content_type' : string,
        'content_encoding' : string,
      },
    ],
    { 'Ok' : boolean } |
      { 'Err' : string }
  >,
}
export declare const idlFactory: IDL.InterfaceFactory;
export declare const init: (args: { IDL: typeof IDL }) => IDL.Type[];
