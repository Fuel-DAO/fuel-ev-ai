import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';
import type { IDL } from '@dfinity/candid';

export interface _SERVICE {
  'add_admin' : ActorMethod<
    [Principal],
    { 'Ok' : boolean } |
      { 'Err' : string }
  >,
  'add_collection_request' : ActorMethod<
    [
      {
        'weight' : number,
        'drive_type' : string,
        'purchase_price' : bigint,
        'token' : Principal,
        'documents' : Array<[string, string]>,
        'supply_cap' : bigint,
        'displays' : string,
        'seating' : string,
        'cargo' : number,
        'logo' : string,
        'name' : string,
        'overall_height' : number,
        'description' : string,
        'overall_width' : number,
        'track_front' : number,
        'ground_clearance' : number,
        'key_features' : Array<string>,
        'range_per_charge' : number,
        'track_rear' : number,
        'acceleration' : string,
        'charging_speed' : string,
        'wheels' : number,
        'brochure_url' : string,
        'index' : Principal,
        'price' : bigint,
        'battery' : string,
        'overall_length' : number,
        'symbol' : string,
        'treasury' : Principal,
        'images' : Array<string>,
      },
    ],
    { 'Ok' : bigint } |
      { 'Err' : string }
  >,
  'approve_request' : ActorMethod<
    [bigint],
    {
        'Ok' : {
          'id' : bigint,
          'token_canister' : Principal,
          'asset_canister' : Principal,
        }
      } |
      { 'Err' : string }
  >,
  'delete_collection' : ActorMethod<
    [bigint],
    { 'Ok' : boolean } |
      { 'Err' : string }
  >,
  'get_asset_canister_wasm' : ActorMethod<
    [],
    {
      'moduleHash' : Uint8Array | number[],
      'chunkHashes' : Array<Uint8Array | number[]>,
    }
  >,
  'get_asset_proxy_canister' : ActorMethod<[], Principal>,
  'get_pending_requests' : ActorMethod<[], Array<bigint>>,
  'get_request_info' : ActorMethod<
    [bigint],
    [] | [
      {
        'metadata' : [] | [
          {
            'weight' : number,
            'drive_type' : string,
            'purchase_price' : bigint,
            'token' : Principal,
            'documents' : Array<[string, string]>,
            'supply_cap' : bigint,
            'displays' : string,
            'seating' : string,
            'cargo' : number,
            'logo' : string,
            'name' : string,
            'overall_height' : number,
            'description' : string,
            'overall_width' : number,
            'track_front' : number,
            'ground_clearance' : number,
            'key_features' : Array<string>,
            'range_per_charge' : number,
            'track_rear' : number,
            'acceleration' : string,
            'charging_speed' : string,
            'wheels' : number,
            'brochure_url' : string,
            'index' : Principal,
            'price' : bigint,
            'battery' : string,
            'overall_length' : number,
            'symbol' : string,
            'treasury' : Principal,
            'images' : Array<string>,
          }
        ],
        'token_canister' : [] | [Principal],
        'approval_status' : { 'Approved' : null } |
          { 'Rejected' : null } |
          { 'Pending' : null },
        'collection_owner' : Principal,
        'asset_canister' : [] | [Principal],
      }
    ]
  >,
  'get_token_canister_wasm' : ActorMethod<
    [],
    {
      'moduleHash' : Uint8Array | number[],
      'chunkHashes' : Array<Uint8Array | number[]>,
    }
  >,
  'is_admin' : ActorMethod<[[] | [Principal]], boolean>,
  'list_collections' : ActorMethod<
    [],
    Array<
      {
        'id' : bigint,
        'token_canister' : Principal,
        'asset_canister' : Principal,
      }
    >
  >,
  'reject_request' : ActorMethod<
    [bigint],
    { 'Ok' : boolean } |
      { 'Err' : string }
  >,
  'remove_admin' : ActorMethod<
    [Principal],
    { 'Ok' : boolean } |
      { 'Err' : string }
  >,
  'set_asset_canister_wasm' : ActorMethod<
    [
      {
        'moduleHash' : Uint8Array | number[],
        'chunkHashes' : Array<Uint8Array | number[]>,
      },
    ],
    { 'Ok' : boolean } |
      { 'Err' : string }
  >,
  'set_asset_proxy_canister' : ActorMethod<
    [Principal],
    { 'Ok' : boolean } |
      { 'Err' : string }
  >,
  'set_token_canister_wasm' : ActorMethod<
    [
      {
        'moduleHash' : Uint8Array | number[],
        'chunkHashes' : Array<Uint8Array | number[]>,
      },
    ],
    { 'Ok' : boolean } |
      { 'Err' : string }
  >,
  'upgrade_token_canister' : ActorMethod<
    [Principal],
    { 'Ok' : boolean } |
      { 'Err' : string }
  >,
  'upgrade_token_canisters' : ActorMethod<
    [],
    { 'Ok' : boolean } |
      { 'Err' : string }
  >,
}
export declare const idlFactory: IDL.InterfaceFactory;
export declare const init: (args: { IDL: typeof IDL }) => IDL.Type[];
