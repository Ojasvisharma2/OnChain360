import { Box } from "@mui/material";
import Navbar from "scenes/navbar";
import UserWidget from "../widgets/UserWidget";
import MyPostWidget from "../widgets/MyPostWidget";
import PostsWidget from "../widgets/PostsWidget";
import AdvertWidget from "../widgets/AdvertWidget";
import FriendList from "../widgets/FriendListWidget";
import { useSelector } from "react-redux";

const HomePage = () => {
  const { _id, picturePath } = useSelector((state) => state.user); // from Redux state

  return (
    <Box>
      <Navbar />
      <Box display="flex" justifyContent="space-between" width="100%" mt="2rem" px="2rem">
        {/* LEFT COLUMN */}
        <Box flexBasis="26%">
          <UserWidget userId={_id} picturePath={picturePath} />
          <FriendList userId={BigInt(_id)} />
        </Box>

        {/* MIDDLE COLUMN */}
        <Box flexBasis="42%">
          <MyPostWidget picturePath={picturePath} />
          <PostsWidget userId={BigInt(_id)} />
        </Box>

        {/* RIGHT COLUMN */}
        <Box flexBasis="26%">
          <AdvertWidget />
        </Box>
      </Box>
    </Box>
  );
};

export default HomePage;