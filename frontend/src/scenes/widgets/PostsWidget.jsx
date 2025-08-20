import { useEffect, useState } from "react";
import { useDispatch } from "react-redux";
import { backend } from "../../ic";
import PostWidget from "./PostWidget";
import { setPosts } from "state";

const PostsWidget = ({ userId }) => {
  const [posts, setPostsState] = useState([]);
  const dispatch = useDispatch();

  const loadPosts = async () => {
    let raw = await backend.list_posts();
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
    setPostsState(enriched);
    dispatch(setPosts({ posts: enriched }));
  };

  useEffect(() => {
    loadPosts();
  }, []);

  return (
    <>
      {posts.map((post) => (
        <PostWidget key={post._id} post={post} reload={loadPosts} />
      ))}
    </>
  );
};

export default PostsWidget;