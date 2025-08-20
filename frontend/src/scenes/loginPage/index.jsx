import { useDispatch } from "react-redux";
import { setLogin } from "state";
import { Button, Box } from "@mui/material";
import { useNavigate } from "react-router-dom";

const LoginPage = () => {
  const dispatch = useDispatch();
  const navigate = useNavigate();

  const fakeLogin = () => {
    // Hardcoded dummy user
    const dummyUser = { _id: "1", username: "demo", picturePath: "" };
    dispatch(setLogin({ user: dummyUser, token: "dummy" }));
    navigate("/home");
  };

  return (
    <Box display="flex" justifyContent="center" alignItems="center" height="100vh">
      <Button variant="contained" onClick={fakeLogin}>
        Login as Demo User
      </Button>
    </Box>
  );
};

export default LoginPage;