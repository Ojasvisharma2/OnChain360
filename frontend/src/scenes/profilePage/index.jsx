import { Box } from "@mui/material";
import { useParams } from "react-router-dom";
import Navbar from "scenes/navbar";
import UserWidget from "../widgets/UserWidget";
import PostsWidget from "../widgets/PostsWidget";

const ProfilePage = () => {
  const { userId } = useParams();

  return (
    <Box>
      <Navbar />
      <Box display="flex" justifyContent="center" mt="2rem">
        <Box flexBasis="60%">
          <UserWidget userId={BigInt(userId)} />
          <PostsWidget userId={BigInt(userId)} />
        </Box>
      </Box>
    </Box>
  );
};

export default ProfilePage;