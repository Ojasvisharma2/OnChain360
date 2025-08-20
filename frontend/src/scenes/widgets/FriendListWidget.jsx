import { useEffect, useState } from "react";
import { backend } from "../../ic";
import { Box, Typography, Button } from "@mui/material";

const FriendList = ({ userId }) => {
  const [users, setUsers] = useState([]);

  const loadUsers = async () => {
    const list = await backend.list_users();
    setUsers(list.filter((u) => u.id !== userId)); // exclude self
  };

  const follow = async (targetId) => {
    await backend.follow(userId, targetId);
    loadUsers();
  };

  const unfollow = async (targetId) => {
    await backend.unfollow(userId, targetId);
    loadUsers();
  };

  useEffect(() => {
    loadUsers();
  }, []);

  return (
    <Box>
      <Typography variant="h6" mb="0.5rem">People you may know</Typography>
      {users.map((u) => (
        <Box key={u.id} display="flex" justifyContent="space-between" alignItems="center" mb="0.5rem">
          <Typography>{u.username}</Typography>
          <Button variant="outlined" size="small" onClick={() => follow(u.id)}>Follow</Button>
        </Box>
      ))}
    </Box>
  );
};

export default FriendList;