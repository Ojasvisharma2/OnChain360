import { useState } from "react";
import { Box, Typography, IconButton, InputBase, useTheme } from "@mui/material";
import { FavoriteBorder, Favorite, ChatBubbleOutline } from "@mui/icons-material";
import { backend } from "../../ic";

const PostWidget = ({ post, reload }) => {
  const { palette } = useTheme();
  const [isLiked, setIsLiked] = useState(false);
  const [comment, setComment] = useState("");

  const likePost = async () => {
    const userId = BigInt(1); // TODO: replace with logged-in user id
    if (isLiked) {
      await backend.unlike_post_api(userId, post._id);
    } else {
      await backend.like_post_api(userId, post._id);
    }
    setIsLiked(!isLiked);
    reload();
  };

  const addComment = async () => {
    const userId = BigInt(1); // TODO: replace with logged-in user id
    await backend.add_comment_api(post._id, userId, comment);
    setComment("");
    reload();
  };

  return (
    <Box border={`1px solid ${palette.neutral.medium}`} borderRadius="0.75rem" p="1rem" mb="1rem">
      <Typography variant="h6" fontWeight="500">{post.firstName}</Typography>
      <Typography>{post.description}</Typography>

      {/* Likes */}
      <IconButton onClick={likePost}>
        {isLiked ? <Favorite sx={{ color: "red" }} /> : <FavoriteBorder />}
      </IconButton>
      <Typography>{post.likes.length} likes</Typography>

      {/* Comments */}
      <Box mt="0.5rem">
        {post.comments.map((c, idx) => (
          <Typography key={idx} sx={{ color: palette.neutral.main }}>
            {c.content}
          </Typography>
        ))}
        <Box display="flex" gap="0.5rem" mt="0.5rem">
          <InputBase
            placeholder="Write a comment..."
            value={comment}
            onChange={(e) => setComment(e.target.value)}
            sx={{ flex: 1, border: `1px solid ${palette.neutral.medium}`, borderRadius: "0.5rem", px: 1 }}
          />
          <IconButton onClick={addComment}>
            <ChatBubbleOutline />
          </IconButton>
        </Box>
      </Box>
    </Box>
  );
};

export default PostWidget;