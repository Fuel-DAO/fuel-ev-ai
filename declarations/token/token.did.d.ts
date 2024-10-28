import type { Principal } from '@dfinity/principal';
import type { ActorMethod } from '@dfinity/agent';
import type { IDL } from '@dfinity/candid';

export interface _SERVICE {
  'accept_sale' : ActorMethod<[], { 'Ok' : boolean } | { 'Err' : string }>,
  'book_tokens' : ActorMethod<
    [{ 'quantity' : bigint }],
    { 'Ok' : boolean } |
      { 'Err' : string }
  >,
  'change_ownership' : ActorMethod<
    [Principal],
    { 'Ok' : bigint } |
      { 'Err' : string }
  >,
  'get_booked_tokens' : ActorMethod<[[] | [Principal]], bigint>,
  'get_escrow_account' : ActorMethod<
    [],
    {
      'accountId' : string,
      'account' : { 'owner' : Principal, 'subaccount' : Uint8Array | number[] },
    }
  >,
  'get_metadata' : ActorMethod<
    [],
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
      'collection_owner' : Principal,
      'asset_canister' : Principal,
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
      'total_supply' : bigint,
      'symbol' : string,
      'treasury' : Principal,
      'images' : Array<string>,
    }
  >,
  'get_sale_status' : ActorMethod<
    [],
    { 'Live' : null } |
      { 'Rejected' : null } |
      { 'Accepted' : null }
  >,
  'get_total_booked_tokens' : ActorMethod<[], bigint>,
  'icrc61_supported_standards' : ActorMethod<
    [],
    Array<{ 'url' : string, 'name' : string }>
  >,
  'icrc7_atomic_batch_transfers' : ActorMethod<[], [] | [boolean]>,
  'icrc7_balance_of' : ActorMethod<
    [
      Array<
        { 'owner' : Principal, 'subaccount' : [] | [Uint8Array | number[]] }
      >,
    ],
    Array<bigint>
  >,
  'icrc7_collection_metadata' : ActorMethod<
    [],
    Array<
      [
        string,
        { 'Int' : bigint } |
          {
            'Map' : Array<
              [
                string,
                { 'Int' : bigint } |
                  { 'Nat' : bigint } |
                  { 'Blob' : Uint8Array | number[] } |
                  { 'Text' : string },
              ]
            >
          } |
          { 'Nat' : bigint } |
          { 'Blob' : Uint8Array | number[] } |
          { 'Text' : string } |
          {
            'Array' : Array<
              { 'Int' : bigint } |
                { 'Nat' : bigint } |
                { 'Blob' : Uint8Array | number[] } |
                { 'Text' : string }
            >
          },
      ]
    >
  >,
  'icrc7_description' : ActorMethod<[], [] | [string]>,
  'icrc7_logo' : ActorMethod<[], [] | [string]>,
  'icrc7_max_default_take_value' : ActorMethod<[], [] | [bigint]>,
  'icrc7_max_memo_size' : ActorMethod<[], [] | [bigint]>,
  'icrc7_max_query_batch_size' : ActorMethod<[], [] | [bigint]>,
  'icrc7_max_take_value' : ActorMethod<[], [] | [bigint]>,
  'icrc7_max_update_batch_size' : ActorMethod<[], [] | [bigint]>,
  'icrc7_name' : ActorMethod<[], string>,
  'icrc7_owner_of' : ActorMethod<
    [Array<bigint>],
    Array<
      [] | [
        { 'owner' : Principal, 'subaccount' : [] | [Uint8Array | number[]] }
      ]
    >
  >,
  'icrc7_permitted_drift' : ActorMethod<[], [] | [bigint]>,
  'icrc7_supply_cap' : ActorMethod<[], [] | [bigint]>,
  'icrc7_symbol' : ActorMethod<[], string>,
  'icrc7_token_metadata' : ActorMethod<
    [Array<bigint>],
    Array<
      [] | [
        Array<
          [
            string,
            { 'Int' : bigint } |
              {
                'Map' : Array<
                  [
                    string,
                    { 'Int' : bigint } |
                      { 'Nat' : bigint } |
                      { 'Blob' : Uint8Array | number[] } |
                      { 'Text' : string },
                  ]
                >
              } |
              { 'Nat' : bigint } |
              { 'Blob' : Uint8Array | number[] } |
              { 'Text' : string } |
              {
                'Array' : Array<
                  { 'Int' : bigint } |
                    { 'Nat' : bigint } |
                    { 'Blob' : Uint8Array | number[] } |
                    { 'Text' : string }
                >
              },
          ]
        >
      ]
    >
  >,
  'icrc7_tokens' : ActorMethod<[[] | [bigint], [] | [bigint]], Array<bigint>>,
  'icrc7_tokens_of' : ActorMethod<
    [
      { 'owner' : Principal, 'subaccount' : [] | [Uint8Array | number[]] },
      [] | [bigint],
      [] | [bigint],
    ],
    Array<bigint>
  >,
  'icrc7_total_supply' : ActorMethod<[], bigint>,
  'icrc7_transfer' : ActorMethod<
    [
      Array<
        {
          'to' : {
            'owner' : Principal,
            'subaccount' : [] | [Uint8Array | number[]],
          },
          'token_id' : bigint,
          'memo' : [] | [Uint8Array | number[]],
          'from_subaccount' : [] | [Uint8Array | number[]],
          'created_at_time' : [] | [bigint],
        }
      >,
    ],
    Array<
      [] | [
        { 'Ok' : bigint } |
          {
            'Err' : {
                'GenericError' : { 'message' : string, 'error_code' : bigint }
              } |
              { 'Duplicate' : { 'duplicate_of' : bigint } } |
              { 'NonExistingTokenId' : null } |
              { 'Unauthorized' : null } |
              { 'CreatedInFuture' : { 'ledger_time' : bigint } } |
              { 'InvalidRecipient' : null } |
              {
                'GenericBatchError' : {
                  'message' : string,
                  'error_code' : bigint,
                }
              } |
              { 'TooOld' : null }
          }
      ]
    >
  >,
  'icrc7_tx_window' : ActorMethod<[], [] | [bigint]>,
  'reject_sale' : ActorMethod<[], { 'Ok' : boolean } | { 'Err' : string }>,
  'reject_sale_individual' : ActorMethod<
    [Principal],
    { 'Ok' : boolean } |
      { 'Err' : string }
  >,
  'update_metadata' : ActorMethod<
    [
      {
        'weight' : [] | [number],
        'drive_type' : [] | [string],
        'purchase_price' : [] | [bigint],
        'token' : [] | [Principal],
        'documents' : [] | [Array<[string, string]>],
        'supply_cap' : [] | [bigint],
        'displays' : [] | [string],
        'seating' : [] | [string],
        'cargo' : [] | [number],
        'logo' : [] | [string],
        'name' : [] | [string],
        'overall_height' : [] | [number],
        'description' : [] | [string],
        'overall_width' : [] | [number],
        'track_front' : [] | [number],
        'asset_canister' : [] | [Principal],
        'ground_clearance' : [] | [number],
        'key_features' : [] | [Array<string>],
        'range_per_charge' : [] | [number],
        'track_rear' : [] | [number],
        'acceleration' : [] | [string],
        'charging_speed' : [] | [string],
        'wheels' : [] | [number],
        'brochure_url' : [] | [string],
        'index' : [] | [Principal],
        'price' : [] | [bigint],
        'battery' : [] | [string],
        'overall_length' : [] | [number],
        'symbol' : [] | [string],
        'treasury' : [] | [Principal],
        'images' : [] | [Array<string>],
      },
    ],
    { 'Ok' : bigint } |
      { 'Err' : string }
  >,
}
export declare const idlFactory: IDL.InterfaceFactory;
export declare const init: (args: { IDL: typeof IDL }) => IDL.Type[];
