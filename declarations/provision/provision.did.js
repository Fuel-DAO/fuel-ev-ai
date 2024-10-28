export const idlFactory = ({ IDL }) => {
  return IDL.Service({
    'add_admin' : IDL.Func(
        [IDL.Principal],
        [IDL.Variant({ 'Ok' : IDL.Bool, 'Err' : IDL.Text })],
        [],
      ),
    'add_collection_request' : IDL.Func(
        [
          IDL.Record({
            'weight' : IDL.Float64,
            'drive_type' : IDL.Text,
            'purchase_price' : IDL.Nat,
            'token' : IDL.Principal,
            'documents' : IDL.Vec(IDL.Tuple(IDL.Text, IDL.Text)),
            'supply_cap' : IDL.Nat,
            'displays' : IDL.Text,
            'seating' : IDL.Text,
            'cargo' : IDL.Float64,
            'logo' : IDL.Text,
            'name' : IDL.Text,
            'overall_height' : IDL.Float64,
            'description' : IDL.Text,
            'overall_width' : IDL.Float64,
            'track_front' : IDL.Float64,
            'ground_clearance' : IDL.Float64,
            'key_features' : IDL.Vec(IDL.Text),
            'range_per_charge' : IDL.Float64,
            'track_rear' : IDL.Float64,
            'acceleration' : IDL.Text,
            'charging_speed' : IDL.Text,
            'wheels' : IDL.Float64,
            'brochure_url' : IDL.Text,
            'index' : IDL.Principal,
            'price' : IDL.Nat,
            'battery' : IDL.Text,
            'overall_length' : IDL.Float64,
            'symbol' : IDL.Text,
            'treasury' : IDL.Principal,
            'images' : IDL.Vec(IDL.Text),
          }),
        ],
        [IDL.Variant({ 'Ok' : IDL.Nat, 'Err' : IDL.Text })],
        [],
      ),
    'approve_request' : IDL.Func(
        [IDL.Nat],
        [
          IDL.Variant({
            'Ok' : IDL.Record({
              'id' : IDL.Nat,
              'token_canister' : IDL.Principal,
              'asset_canister' : IDL.Principal,
            }),
            'Err' : IDL.Text,
          }),
        ],
        [],
      ),
    'delete_collection' : IDL.Func(
        [IDL.Nat],
        [IDL.Variant({ 'Ok' : IDL.Bool, 'Err' : IDL.Text })],
        [],
      ),
    'get_asset_canister_wasm' : IDL.Func(
        [],
        [
          IDL.Record({
            'moduleHash' : IDL.Vec(IDL.Nat8),
            'chunkHashes' : IDL.Vec(IDL.Vec(IDL.Nat8)),
          }),
        ],
        ['query'],
      ),
    'get_asset_proxy_canister' : IDL.Func([], [IDL.Principal], ['query']),
    'get_pending_requests' : IDL.Func([], [IDL.Vec(IDL.Nat)], ['query']),
    'get_request_info' : IDL.Func(
        [IDL.Nat],
        [
          IDL.Opt(
            IDL.Record({
              'metadata' : IDL.Opt(
                IDL.Record({
                  'weight' : IDL.Float64,
                  'drive_type' : IDL.Text,
                  'purchase_price' : IDL.Nat,
                  'token' : IDL.Principal,
                  'documents' : IDL.Vec(IDL.Tuple(IDL.Text, IDL.Text)),
                  'supply_cap' : IDL.Nat,
                  'displays' : IDL.Text,
                  'seating' : IDL.Text,
                  'cargo' : IDL.Float64,
                  'logo' : IDL.Text,
                  'name' : IDL.Text,
                  'overall_height' : IDL.Float64,
                  'description' : IDL.Text,
                  'overall_width' : IDL.Float64,
                  'track_front' : IDL.Float64,
                  'ground_clearance' : IDL.Float64,
                  'key_features' : IDL.Vec(IDL.Text),
                  'range_per_charge' : IDL.Float64,
                  'track_rear' : IDL.Float64,
                  'acceleration' : IDL.Text,
                  'charging_speed' : IDL.Text,
                  'wheels' : IDL.Float64,
                  'brochure_url' : IDL.Text,
                  'index' : IDL.Principal,
                  'price' : IDL.Nat,
                  'battery' : IDL.Text,
                  'overall_length' : IDL.Float64,
                  'symbol' : IDL.Text,
                  'treasury' : IDL.Principal,
                  'images' : IDL.Vec(IDL.Text),
                })
              ),
              'token_canister' : IDL.Opt(IDL.Principal),
              'approval_status' : IDL.Variant({
                'Approved' : IDL.Null,
                'Rejected' : IDL.Null,
                'Pending' : IDL.Null,
              }),
              'collection_owner' : IDL.Principal,
              'asset_canister' : IDL.Opt(IDL.Principal),
            })
          ),
        ],
        ['query'],
      ),
    'get_token_canister_wasm' : IDL.Func(
        [],
        [
          IDL.Record({
            'moduleHash' : IDL.Vec(IDL.Nat8),
            'chunkHashes' : IDL.Vec(IDL.Vec(IDL.Nat8)),
          }),
        ],
        ['query'],
      ),
    'is_admin' : IDL.Func([IDL.Opt(IDL.Principal)], [IDL.Bool], ['query']),
    'list_collections' : IDL.Func(
        [],
        [
          IDL.Vec(
            IDL.Record({
              'id' : IDL.Nat,
              'token_canister' : IDL.Principal,
              'asset_canister' : IDL.Principal,
            })
          ),
        ],
        ['query'],
      ),
    'reject_request' : IDL.Func(
        [IDL.Nat],
        [IDL.Variant({ 'Ok' : IDL.Bool, 'Err' : IDL.Text })],
        [],
      ),
    'remove_admin' : IDL.Func(
        [IDL.Principal],
        [IDL.Variant({ 'Ok' : IDL.Bool, 'Err' : IDL.Text })],
        [],
      ),
    'set_asset_canister_wasm' : IDL.Func(
        [
          IDL.Record({
            'moduleHash' : IDL.Vec(IDL.Nat8),
            'chunkHashes' : IDL.Vec(IDL.Vec(IDL.Nat8)),
          }),
        ],
        [IDL.Variant({ 'Ok' : IDL.Bool, 'Err' : IDL.Text })],
        [],
      ),
    'set_asset_proxy_canister' : IDL.Func(
        [IDL.Principal],
        [IDL.Variant({ 'Ok' : IDL.Bool, 'Err' : IDL.Text })],
        [],
      ),
    'set_token_canister_wasm' : IDL.Func(
        [
          IDL.Record({
            'moduleHash' : IDL.Vec(IDL.Nat8),
            'chunkHashes' : IDL.Vec(IDL.Vec(IDL.Nat8)),
          }),
        ],
        [IDL.Variant({ 'Ok' : IDL.Bool, 'Err' : IDL.Text })],
        [],
      ),
    'upgrade_token_canister' : IDL.Func(
        [IDL.Principal],
        [IDL.Variant({ 'Ok' : IDL.Bool, 'Err' : IDL.Text })],
        [],
      ),
    'upgrade_token_canisters' : IDL.Func(
        [],
        [IDL.Variant({ 'Ok' : IDL.Bool, 'Err' : IDL.Text })],
        [],
      ),
  });
};
export const init = ({ IDL }) => {
  return [IDL.Opt(IDL.Variant({ 'Upgrade' : IDL.Null, 'Init' : IDL.Null }))];
};
