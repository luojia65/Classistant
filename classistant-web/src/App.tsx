import React from "react";
import Container from "@material-ui/core/Container";
import Typography from "@material-ui/core/Typography";
import Box from "@material-ui/core/Box";
import Link from "@material-ui/core/Link";
import LoginBox from "./LoginBox";
import ButtonAppBar from "./AppBar";
import { Grid } from "@material-ui/core";

function Copyright() {
  return (
    <Typography variant="body2" color="textSecondary" align="center">
      {"Copyright © "}
      <Link color="inherit" href="https://material-ui.com/">
        ClassAssistant
      </Link>{" "}
      {new Date().getFullYear()}
      {"."}
    </Typography>
  );
}

export default function App() {
  return (
    <Container maxWidth="xl">
      <Grid >
        <ButtonAppBar />
        <Typography variant="h4" component="h1" gutterBottom>
          ClassAssistant
        </Typography>
        <LoginBox />
        <Copyright />
      </Grid>
    </Container>
  );
}
