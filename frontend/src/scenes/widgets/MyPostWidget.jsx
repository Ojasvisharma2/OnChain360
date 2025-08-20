// ...imports stay the same...
import { backend } from "../../ic";

const MyPostWidget = ({ picturePath }) => {
  // ...state and hooks unchanged...

  const handlePost = async () => {
    // your Redux user likely has `id` or `_id`; adapt as needed:
    const authorId = BigInt(_id); // ensure BigInt for u64 candid
    const content = post;

    // create post on-chain
    await backend.create_post_api(authorId, content);

    // refresh feed (use the same loader logic your PostsWidget uses)
    const raw = await backend.list_posts();
    const enriched = await Promise.all(
      raw.map(async (p) => {
        const author = await backend.get_user(p.author_id);
        return {
          _id: p.id,
          userId: p.author_id,
          firstName: author?.username ?? "",
          lastName: "",
          description: p.content,
          location: "",
          picturePath: "",
          userPicturePath: "",
          likes: p.likes || [],
          comments: p.comments || [],
          timestamp: p.timestamp,
        };
      })
    );
    dispatch(setPosts({ posts: enriched }));

    // reset UI
    setImage(null);
    setPost("");
  };

  // ...rest of component unchanged...
};

export default MyPostWidget;