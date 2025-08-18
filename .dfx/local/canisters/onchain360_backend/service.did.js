export const idlFactory = ({ IDL }) => {
  const AuthResult = IDL.Record({
    'message' : IDL.Opt(IDL.Text),
    'success' : IDL.Bool,
  });
  const Comment = IDL.Record({
    'id' : IDL.Nat64,
    'post_id' : IDL.Nat64,
    'content' : IDL.Text,
    'timestamp' : IDL.Nat64,
    'author_id' : IDL.Nat64,
  });
  const DirectMessage = IDL.Record({
    'id' : IDL.Nat64,
    'receiver_id' : IDL.Nat64,
    'content' : IDL.Text,
    'sender_id' : IDL.Nat64,
    'timestamp' : IDL.Nat64,
  });
  const Notification = IDL.Record({
    'id' : IDL.Nat64,
    'read' : IDL.Bool,
    'user_id' : IDL.Nat64,
    'message' : IDL.Text,
    'timestamp' : IDL.Nat64,
  });
  const Post = IDL.Record({
    'id' : IDL.Nat64,
    'content' : IDL.Text,
    'likes' : IDL.Vec(IDL.Nat64),
    'timestamp' : IDL.Nat64,
    'author_id' : IDL.Nat64,
    'comments' : IDL.Vec(IDL.Nat64),
  });
  const User = IDL.Record({
    'id' : IDL.Nat64,
    'bio' : IDL.Opt(IDL.Text),
    'username' : IDL.Text,
    'followers' : IDL.Vec(IDL.Nat64),
    'following' : IDL.Vec(IDL.Nat64),
  });
  const Result = IDL.Variant({ 'Ok' : IDL.Text, 'Err' : AuthResult });
  const Result_1 = IDL.Variant({ 'Ok' : User, 'Err' : AuthResult });
  return IDL.Service({
    'add_comment_api' : IDL.Func(
        [IDL.Nat64, IDL.Nat64, IDL.Text],
        [IDL.Nat64],
        [],
      ),
    'add_notification_api' : IDL.Func([IDL.Nat64, IDL.Text], [IDL.Nat64], []),
    'confirm_reset' : IDL.Func(
        [IDL.Text, IDL.Text, IDL.Text],
        [AuthResult],
        [],
      ),
    'create_post_api' : IDL.Func([IDL.Nat64, IDL.Text], [IDL.Nat64], []),
    'follow' : IDL.Func([IDL.Nat64, IDL.Nat64], [IDL.Bool], []),
    'get_comment' : IDL.Func([IDL.Nat64], [IDL.Opt(Comment)], ['query']),
    'get_dms_api' : IDL.Func([IDL.Nat64], [IDL.Vec(DirectMessage)], ['query']),
    'get_notifications_api' : IDL.Func(
        [IDL.Nat64],
        [IDL.Vec(Notification)],
        ['query'],
      ),
    'get_post_api' : IDL.Func([IDL.Nat64], [IDL.Opt(Post)], ['query']),
    'get_post_likes' : IDL.Func([IDL.Nat64], [IDL.Vec(IDL.Nat64)], ['query']),
    'get_user' : IDL.Func([IDL.Nat64], [IDL.Opt(User)], ['query']),
    'like_post_api' : IDL.Func([IDL.Nat64, IDL.Nat64], [IDL.Bool], []),
    'list_post_comments' : IDL.Func([IDL.Nat64], [IDL.Vec(Comment)], ['query']),
    'list_users' : IDL.Func([], [IDL.Vec(User)], ['query']),
    'login_user' : IDL.Func([IDL.Text, IDL.Text], [AuthResult], []),
    'request_reset' : IDL.Func([IDL.Text], [Result], []),
    'send_dm_api' : IDL.Func([IDL.Nat64, IDL.Nat64, IDL.Text], [IDL.Nat64], []),
    'signup_user' : IDL.Func(
        [IDL.Text, IDL.Text, IDL.Text, IDL.Opt(IDL.Text)],
        [Result_1],
        [],
      ),
    'unfollow' : IDL.Func([IDL.Nat64, IDL.Nat64], [IDL.Bool], []),
    'unlike_post_api' : IDL.Func([IDL.Nat64, IDL.Nat64], [IDL.Bool], []),
  });
};
export const init = ({ IDL }) => { return []; };
