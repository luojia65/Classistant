CREATE TABLE `DUserAuth` (
  `user_id` int(11) NOT NULL AUTO_INCREMENT,
  `nickname` varchar(48) NOT NULL,
  `hash` blob NOT NULL,
  `date_register` datetime NOT NULL DEFAULT CURRENT_TIMESTAMP,
  PRIMARY KEY (`user_id`)
);

CREATE PROCEDURE `PUserRegister`(
	IN `_nickname` VARCHAR(48),
	IN `_hash` BLOB,
  OUT `_return_id` INT,
  OUT `_user_id` INT
)
BEGIN
	DECLARE `_tmp_user_id` varchar(48);
    
	SELECT `user_id` 
    INTO `_tmp_user_id`
    FROM `DUserAuth` WHERE `nickname` = `_nickname`;
    
    IF `_tmp_user_id` IS NOT NULL THEN 
		SET `_return_id` = 1; -- nickname already taken
	ELSE
		INSERT INTO `DUserAuth` (`nickname`,`hash`) 
        VALUES (`_nickname`,`hash`);
        SELECT `user_id` INTO `_tmp_user_id` FROM `DUserAuth` WHERE `nickname` = `_nickname`;
        SET `_user_id` = `_tmp_user_id`;
        SET `_return_id` = 0; -- success
    END IF;
END
