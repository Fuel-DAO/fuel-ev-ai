export const idlFactory = ({ IDL }) => {
  return IDL.Service({
    'accept_sale' : IDL.Func(
        [],
        [IDL.Variant({ 'Ok' : IDL.Bool, 'Err' : IDL.Text })],
        [],
      ),
    'book_tokens' : IDL.Func(
        [IDL.Record({ 'quantity' : IDL.Nat })],
        [IDL.Variant({ 'Ok' : IDL.Bool, 'Err' : IDL.Text })],
        [],
      ),
    'change_ownership' : IDL.Func(
        [IDL.Principal],
        [IDL.Variant({ 'Ok' : IDL.Nat, 'Err' : IDL.Text })],
        [],
      ),
    'get_booked_tokens' : IDL.Func(
        [IDL.Opt(IDL.Principal)],
        [IDL.Nat],
        ['query'],
      ),
    'get_escrow_account' : IDL.Func(
        [],
        [
          IDL.Record({
            'accountId' : IDL.Text,
            'account' : IDL.Record({
              'owner' : IDL.Principal,
              'subaccount' : IDL.Vec(IDL.Nat8),
            }),
          }),
        ],
        ['query'],
      ),
    'get_metadata' : IDL.Func(
        [],
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
            'collection_owner' : IDL.Principal,
            'asset_canister' : IDL.Principal,
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
            'total_supply' : IDL.Nat,
            'symbol' : IDL.Text,
            'treasury' : IDL.Principal,
            'images' : IDL.Vec(IDL.Text),
          }),
        ],
        ['query'],
      ),
    'get_sale_status' : IDL.Func(
        [],
        [
          IDL.Variant({
            'Live' : IDL.Null,
            'Rejected' : IDL.Null,
            'Accepted' : IDL.Null,
          }),
        ],
        ['query'],
      ),
    'get_total_booked_tokens' : IDL.Func([], [IDL.Nat], ['query']),
    'icrc61_supported_standards' : IDL.Func(
        [],
        [IDL.Vec(IDL.Record({ 'url' : IDL.Text, 'name' : IDL.Text }))],
        ['query'],
      ),
    'icrc7_atomic_batch_transfers' : IDL.Func(
        [],
        [IDL.Opt(IDL.Bool)],
        ['query'],
      ),
    'icrc7_balance_of' : IDL.Func(
        [
          IDL.Vec(
            IDL.Record({
              'owner' : IDL.Principal,
              'subaccount' : IDL.Opt(IDL.Vec(IDL.Nat8)),
            })
          ),
        ],
        [IDL.Vec(IDL.Nat)],
        ['query'],
      ),
    'icrc7_collection_metadata' : IDL.Func(
        [],
        [
          IDL.Vec(
            IDL.Tuple(
              IDL.Text,
              IDL.Variant({
                'Int' : IDL.Int,
                'Map' : IDL.Vec(
                  IDL.Tuple(
                    IDL.Text,
                    IDL.Variant({
                      'Int' : IDL.Int,
                      'Nat' : IDL.Nat,
                      'Blob' : IDL.Vec(IDL.Nat8),
                      'Text' : IDL.Text,
                    }),
                  )
                ),
                'Nat' : IDL.Nat,
                'Blob' : IDL.Vec(IDL.Nat8),
                'Text' : IDL.Text,
                'Array' : IDL.Vec(
                  IDL.Variant({
                    'Int' : IDL.Int,
                    'Nat' : IDL.Nat,
                    'Blob' : IDL.Vec(IDL.Nat8),
                    'Text' : IDL.Text,
                  })
                ),
              }),
            )
          ),
        ],
        ['query'],
      ),
    'icrc7_description' : IDL.Func([], [IDL.Opt(IDL.Text)], ['query']),
    'icrc7_logo' : IDL.Func([], [IDL.Opt(IDL.Text)], ['query']),
    'icrc7_max_default_take_value' : IDL.Func(
        [],
        [IDL.Opt(IDL.Nat)],
        ['query'],
      ),
    'icrc7_max_memo_size' : IDL.Func([], [IDL.Opt(IDL.Nat)], ['query']),
    'icrc7_max_query_batch_size' : IDL.Func([], [IDL.Opt(IDL.Nat)], ['query']),
    'icrc7_max_take_value' : IDL.Func([], [IDL.Opt(IDL.Nat)], ['query']),
    'icrc7_max_update_batch_size' : IDL.Func([], [IDL.Opt(IDL.Nat)], ['query']),
    'icrc7_name' : IDL.Func([], [IDL.Text], ['query']),
    'icrc7_owner_of' : IDL.Func(
        [IDL.Vec(IDL.Nat)],
        [
          IDL.Vec(
            IDL.Opt(
              IDL.Record({
                'owner' : IDL.Principal,
                'subaccount' : IDL.Opt(IDL.Vec(IDL.Nat8)),
              })
            )
          ),
        ],
        ['query'],
      ),
    'icrc7_permitted_drift' : IDL.Func([], [IDL.Opt(IDL.Nat)], ['query']),
    'icrc7_supply_cap' : IDL.Func([], [IDL.Opt(IDL.Nat)], ['query']),
    'icrc7_symbol' : IDL.Func([], [IDL.Text], ['query']),
    'icrc7_token_metadata' : IDL.Func(
        [IDL.Vec(IDL.Nat)],
        [
          IDL.Vec(
            IDL.Opt(
              IDL.Vec(
                IDL.Tuple(
                  IDL.Text,
                  IDL.Variant({
                    'Int' : IDL.Int,
                    'Map' : IDL.Vec(
                      IDL.Tuple(
                        IDL.Text,
                        IDL.Variant({
                          'Int' : IDL.Int,
                          'Nat' : IDL.Nat,
                          'Blob' : IDL.Vec(IDL.Nat8),
                          'Text' : IDL.Text,
                        }),
                      )
                    ),
                    'Nat' : IDL.Nat,
                    'Blob' : IDL.Vec(IDL.Nat8),
                    'Text' : IDL.Text,
                    'Array' : IDL.Vec(
                      IDL.Variant({
                        'Int' : IDL.Int,
                        'Nat' : IDL.Nat,
                        'Blob' : IDL.Vec(IDL.Nat8),
                        'Text' : IDL.Text,
                      })
                    ),
                  }),
                )
              )
            )
          ),
        ],
        ['query'],
      ),
    'icrc7_tokens' : IDL.Func(
        [IDL.Opt(IDL.Nat), IDL.Opt(IDL.Nat)],
        [IDL.Vec(IDL.Nat)],
        ['query'],
      ),
    'icrc7_tokens_of' : IDL.Func(
        [
          IDL.Record({
            'owner' : IDL.Principal,
            'subaccount' : IDL.Opt(IDL.Vec(IDL.Nat8)),
          }),
          IDL.Opt(IDL.Nat),
          IDL.Opt(IDL.Nat),
        ],
        [IDL.Vec(IDL.Nat)],
        ['query'],
      ),
    'icrc7_total_supply' : IDL.Func([], [IDL.Nat], ['query']),
    'icrc7_transfer' : IDL.Func(
        [
          IDL.Vec(
            IDL.Record({
              'to' : IDL.Record({
                'owner' : IDL.Principal,
                'subaccount' : IDL.Opt(IDL.Vec(IDL.Nat8)),
              }),
              'token_id' : IDL.Nat,
              'memo' : IDL.Opt(IDL.Vec(IDL.Nat8)),
              'from_subaccount' : IDL.Opt(IDL.Vec(IDL.Nat8)),
              'created_at_time' : IDL.Opt(IDL.Nat64),
            })
          ),
        ],
        [
          IDL.Vec(
            IDL.Opt(
              IDL.Variant({
                'Ok' : IDL.Nat,
                'Err' : IDL.Variant({
                  'GenericError' : IDL.Record({
                    'message' : IDL.Text,
                    'error_code' : IDL.Nat,
                  }),
                  'Duplicate' : IDL.Record({ 'duplicate_of' : IDL.Nat }),
                  'NonExistingTokenId' : IDL.Null,
                  'Unauthorized' : IDL.Null,
                  'CreatedInFuture' : IDL.Record({ 'ledger_time' : IDL.Nat64 }),
                  'InvalidRecipient' : IDL.Null,
                  'GenericBatchError' : IDL.Record({
                    'message' : IDL.Text,
                    'error_code' : IDL.Nat,
                  }),
                  'TooOld' : IDL.Null,
                }),
              })
            )
          ),
        ],
        [],
      ),
    'icrc7_tx_window' : IDL.Func([], [IDL.Opt(IDL.Nat)], ['query']),
    'reject_sale' : IDL.Func(
        [],
        [IDL.Variant({ 'Ok' : IDL.Bool, 'Err' : IDL.Text })],
        [],
      ),
    'reject_sale_individual' : IDL.Func(
        [IDL.Principal],
        [IDL.Variant({ 'Ok' : IDL.Bool, 'Err' : IDL.Text })],
        [],
      ),
    'update_metadata' : IDL.Func(
        [
          IDL.Record({
            'weight' : IDL.Opt(IDL.Float64),
            'drive_type' : IDL.Opt(IDL.Text),
            'purchase_price' : IDL.Opt(IDL.Nat),
            'token' : IDL.Opt(IDL.Principal),
            'documents' : IDL.Opt(IDL.Vec(IDL.Tuple(IDL.Text, IDL.Text))),
            'supply_cap' : IDL.Opt(IDL.Nat),
            'displays' : IDL.Opt(IDL.Text),
            'seating' : IDL.Opt(IDL.Text),
            'cargo' : IDL.Opt(IDL.Float64),
            'logo' : IDL.Opt(IDL.Text),
            'name' : IDL.Opt(IDL.Text),
            'overall_height' : IDL.Opt(IDL.Float64),
            'description' : IDL.Opt(IDL.Text),
            'overall_width' : IDL.Opt(IDL.Float64),
            'track_front' : IDL.Opt(IDL.Float64),
            'asset_canister' : IDL.Opt(IDL.Principal),
            'ground_clearance' : IDL.Opt(IDL.Float64),
            'key_features' : IDL.Opt(IDL.Vec(IDL.Text)),
            'range_per_charge' : IDL.Opt(IDL.Float64),
            'track_rear' : IDL.Opt(IDL.Float64),
            'acceleration' : IDL.Opt(IDL.Text),
            'charging_speed' : IDL.Opt(IDL.Text),
            'wheels' : IDL.Opt(IDL.Float64),
            'brochure_url' : IDL.Opt(IDL.Text),
            'index' : IDL.Opt(IDL.Principal),
            'price' : IDL.Opt(IDL.Nat),
            'battery' : IDL.Opt(IDL.Text),
            'overall_length' : IDL.Opt(IDL.Float64),
            'symbol' : IDL.Opt(IDL.Text),
            'treasury' : IDL.Opt(IDL.Principal),
            'images' : IDL.Opt(IDL.Vec(IDL.Text)),
          }),
        ],
        [IDL.Variant({ 'Ok' : IDL.Nat, 'Err' : IDL.Text })],
        [],
      ),
  });
};
export const init = ({ IDL }) => {
  return [
    IDL.Variant({
      'Upgrade' : IDL.Null,
      'Init' : IDL.Record({
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
        'collection_owner' : IDL.Principal,
        'asset_canister' : IDL.Principal,
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
    }),
  ];
};
