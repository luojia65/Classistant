CREATE TABLE `DUserAuth` (
  `user_id` int(11) NOT NULL AUTO_INCREMENT,
  `nickname` varchar(48) NOT NULL,
  `hash` blob NOT NULL,
  `date_register` datetime NOT NULL DEFAULT CURRENT_TIMESTAMP,
  PRIMARY KEY (`user_id`)
);

CREATE PROCEDURE `PUserRegister`(
	IN `_nickname` VARCHAR(48),
	IN `_hash` BLOB
)
BEGIN
	DECLARE `_tmp_user_id` varchar(48);
  DECLARE `_return_id` INT;
    
	SELECT `user_id` 
  INTO `_tmp_user_id`
  FROM `DUserAuth` WHERE `nickname` = `_nickname`;
    
  IF `_tmp_user_id` IS NOT NULL THEN 
    SELECT 1 as `return_id`; -- nickname already taken
	ELSE
    INSERT INTO `DUserAuth` (`nickname`,`hash`) 
    VALUES (`_nickname`,`hash`);
    SELECT `user_id` INTO `_tmp_user_id` FROM `DUserAuth` WHERE `nickname` = `_nickname`;
    SELECT 0 as `return_id`, `_tmp_user_id` as `user_id`; -- success
  END IF;
END
