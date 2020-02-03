import React from "react";
import TextField from "@material-ui/core/TextField";
import {makeStyles} from "@material-ui/core/styles";
import {Avatar, Button, Container} from "@material-ui/core";
import DeleteIcon from '@material-ui/icons/Delete';

const useStyles = makeStyles(theme => ({
        form: {
            width: "100%", // Fix IE 11 issue.
            marginTop: theme.spacing(1),
        },
        large: {
            width: theme.spacing(7),
            height: theme.spacing(7),
            margin: "auto"
        },
        button: {
            margin: theme.spacing(2),
            marginTop: theme.spacing(3)
        }
    }))
;
export default function AddGroup() {
    const classes = useStyles();
    return (
        <Container>
            <div>
                <form className={classes.form}>
                    <Avatar className={classes.large}>G</Avatar>
                    <TextField
                        margin="normal" required id="groupName" label="群组名称" autoFocus/>
                    <TextField fullWidth margin="normal" required id="groupDes" label="群组描述" rowsMax={4}
                               autoFocus
                               multiline/>
                </form>
                <Button
                    variant="contained"
                    color="secondary"
                    className={classes.button}
                    // style={{marginRight:}}
                    startIcon={<DeleteIcon/>}>
                    取消
                </Button>
                <Button
                    variant="contained"
                    color="primary"
                    className={classes.button}>
                    创建
                </Button>
            </div>
        </Container>
    );
}