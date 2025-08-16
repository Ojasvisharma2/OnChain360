export const idlFactory = ({ IDL }) => {
  return IDL.Service({
    'add_post' : IDL.Func([IDL.Text], [], []),
    'add_user' : IDL.Func([IDL.Text], [], []),
    'get_posts' : IDL.Func([], [IDL.Text], []),
    'get_users' : IDL.Func([], [IDL.Text], []),
    'greet' : IDL.Func([IDL.Text], [IDL.Text], []),
  });
};
export const init = ({ IDL }) => { return []; };
