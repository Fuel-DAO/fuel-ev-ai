export const idlFactory = ({ IDL }) => {
  return IDL.Service({
    'approve_files' : IDL.Func(
        [
          IDL.Record({
            'files' : IDL.Vec(IDL.Text),
            'asset_canister' : IDL.Principal,
          }),
        ],
        [IDL.Variant({ 'Ok' : IDL.Bool, 'Err' : IDL.Text })],
        [],
      ),
    'get_provision_canister' : IDL.Func([], [IDL.Principal], ['query']),
    'get_temp_asset_canister' : IDL.Func([], [IDL.Principal], ['query']),
    'prune' : IDL.Func(
        [IDL.Vec(IDL.Text)],
        [IDL.Variant({ 'Ok' : IDL.Bool, 'Err' : IDL.Text })],
        [],
      ),
    'reject_files' : IDL.Func(
        [IDL.Vec(IDL.Text)],
        [IDL.Variant({ 'Ok' : IDL.Bool, 'Err' : IDL.Text })],
        [],
      ),
    'set_provision_canister' : IDL.Func(
        [IDL.Principal],
        [IDL.Variant({ 'Ok' : IDL.Bool, 'Err' : IDL.Text })],
        [],
      ),
    'set_temp_asset_canister' : IDL.Func(
        [IDL.Principal],
        [IDL.Variant({ 'Ok' : IDL.Bool, 'Err' : IDL.Text })],
        [],
      ),
    'store' : IDL.Func(
        [
          IDL.Record({
            'key' : IDL.Text,
            'content' : IDL.Vec(IDL.Nat8),
            'sha256' : IDL.Opt(IDL.Vec(IDL.Nat8)),
            'content_type' : IDL.Text,
            'content_encoding' : IDL.Text,
          }),
        ],
        [IDL.Variant({ 'Ok' : IDL.Bool, 'Err' : IDL.Text })],
        [],
      ),
  });
};
export const init = ({ IDL }) => {
  return [IDL.Opt(IDL.Variant({ 'Upgrade' : IDL.Null, 'Init' : IDL.Null }))];
};
