import React from "react";
import PhoneIcon from "@material-ui/icons/Phone";
import AccountCircle from "@material-ui/icons/AccountCircle";
import { makeStyles, Grid, TextField } from "@material-ui/core";
import Title from "./Title";

const useStyles = makeStyles(theme => ({
  textField: {
    margin: theme.spacing(1),
    justifyContent: "center"
  }
}));

export default function Informations() {
  const classes = useStyles();
  return (
    <React.Fragment>
      <Title>用户资料</Title>
      <Grid container>
        <Grid className={classes.textField} container alignItems="flex-end">
          <Grid item>
            <AccountCircle></AccountCircle>
          </Grid>
          <Grid item>
            <TextField label="姓名"></TextField>
          </Grid>
        </Grid>
        <Grid className={classes.textField} container alignItems="flex-end">
          <Grid item>
            <PhoneIcon></PhoneIcon>
          </Grid>
          <Grid item>
            <TextField label="电话"></TextField>
          </Grid>
        </Grid>
      </Grid>
    </React.Fragment>
  );
}
