import Visibility from "@material-ui/icons/Visibility";
import clsx from "clsx";
import VisibilityOff from "@material-ui/icons/VisibilityOff";
import React, { useEffect } from "react";
import Input from "@material-ui/core/Input";
import { makeStyles } from "@material-ui/styles";
import {
  Theme,
  createStyles,
  FormControl,
  InputLabel,
  InputAdornment,
  IconButton,
  Tooltip,
  Button
} from "@material-ui/core";

const useStyles = makeStyles((theme: Theme) =>
  createStyles({
    root: {
      display: "flex",
      flexWrap: "wrap",
      flexDirection: "column"
    },
    margin: {
      margin: theme.spacing(1),
      width: 250
    },
    withoutLabel: {
      marginTop: theme.spacing(3)
    },
    textField: {
      flexBasis: 200
    }
  })
);

interface State {
  phoneNum: string;
  authCode: string;
  password: string;
  canAuth: boolean;
  showPassword: boolean;
}

interface CanAuthProps {
  canAuth: boolean;
  handleClick: () => void;
}

function AuthTimeButton(props: CanAuthProps) {
  const [time, setTime] = React.useState<number>(10); //can hook place in here?
  useEffect(() => {
    if (!props.canAuth) {
      if (time <= 0) {
        props.handleClick();
        setTime(10);
        return;
      }
      setTimeout(() => {
        setTime(time - 1);
      }, 1000);
      console.log(time);
    }
  }, [time, props]);
  if (props.canAuth) {
    return (
      <Tooltip title="Auth">
        <Button
          onClick={() => {
            props.handleClick();
            //TODO
          }}
        >
          Auth
        </Button>
      </Tooltip>
    );
  } else {
    return (
      <Tooltip title="Auth">
        <Button>{time}s Remain</Button>
      </Tooltip>
    );
  }
}

export default function LoginBox() {
  const classes = useStyles();
  const [values, setValues] = React.useState<State>({
    phoneNum: "",
    authCode: "",
    password: "",
    showPassword: false,
    canAuth: true
  });

  const handleChange = (prop: keyof State) => (
    event: React.ChangeEvent<HTMLInputElement>
  ) => {
    setValues({ ...values, [prop]: event.target.value });
  };
  const handleClickShowPassword = () => {
    setValues({ ...values, showPassword: !values.showPassword });
  };

  const handleMouseDownPassword = (
    event: React.MouseEvent<HTMLButtonElement>
  ) => {
    event.preventDefault();
  };

  const handleClickAuthCode = () => {
    setValues({ ...values, canAuth: !values.canAuth });
    console.log("AUTH CLICK!");
    //TODO
  };

  return (
    <div className={classes.root}>
      <FormControl className={classes.margin}>
        <InputLabel htmlFor="adornment-phone">phoneNumber</InputLabel>
        <Input
          id="adornment-phone"
          value={values.phoneNum}
          onChange={handleChange("phoneNum")}
          startAdornment={<InputAdornment position="start">+86</InputAdornment>}
        ></Input>
      </FormControl>
      <FormControl className={classes.margin}>
        <InputLabel htmlFor="adornment-authcode">authCode</InputLabel>
        <Input
          id="adornment-code"
          value={values.authCode}
          onChange={handleChange("authCode")}
          startAdornment={<InputAdornment position="start"></InputAdornment>}
          endAdornment={
            <AuthTimeButton
              canAuth={values.canAuth}
              handleClick={handleClickAuthCode}
            />
          }
        ></Input>
      </FormControl>
      <FormControl className={clsx(classes.margin, classes.textField)}>
        <InputLabel htmlFor="adornment-password">Password</InputLabel>
        <Input
          id="adornment-password"
          type={values.showPassword ? "text" : "password"}
          value={values.password}
          onChange={handleChange("password")}
          startAdornment={<InputAdornment position="start"></InputAdornment>}
          endAdornment={
            <InputAdornment position="end">
              <IconButton
                aria-label="toggle password visibility"
                onClick={handleClickShowPassword}
                onMouseDown={handleMouseDownPassword}
              >
                {values.showPassword ? <Visibility /> : <VisibilityOff />}
              </IconButton>
            </InputAdornment>
          }
        />
      </FormControl>
    </div>
  );
}
