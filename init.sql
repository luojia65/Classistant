-- previously used names
DROP PROCEDURE IF EXISTS `PUserRegister`;
DROP PROCEDURE IF EXISTS `PUserLoginById`;

-- tables & procedures
CREATE TABLE IF NOT EXISTS `DUserAuth` (
    `user_id` int(11) NOT NULL AUTO_INCREMENT,
    `hash` blob,
    `date_register` datetime NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (`user_id`)
);

DROP PROCEDURE IF EXISTS `PUserRegisterByNickname`;

CREATE PROCEDURE `PUserRegisterByNickname`(
    IN `_nickname` VARBINARY(256)
)
BEGIN
    DECLARE `_tmp_user_id` INT;
    
    SELECT `user_id` INTO `_tmp_user_id`
    FROM `DAuthId` 
    WHERE `auth_type` = 'nickname' AND `auth_id` = `_nickname`;
    
    IF `_tmp_user_id` IS NOT NULL THEN 
        SELECT 1 as `return_id`; -- nickname already taken
    ELSE
        INSERT INTO `DAuthHash` (`hash`) VALUES (NULL);
		SELECT last_insert_id() INTO `_tmp_user_id`;
		INSERT INTO `DAuthId` (`user_id`, `auth_type`, `auth_id`)
        VALUES (`_tmp_user_id`, 'nickname', `_nickname`);
        SELECT 0 as `return_id`, `_tmp_user_id` as `user_id`; -- success
    END IF;
END

DROP PROCEDURE IF EXISTS `PUserRegisterFillHash`;

CREATE PROCEDURE `PUserRegisterFillHash`(
    IN `_user_id` INT,
    IN `_hash` BLOB
)
BEGIN 
    UPDATE `DAuthHash` SET `hash` = `_hash`
    WHERE `user_id` = `_user_id`;
END

DROP PROCEDURE IF EXISTS `PUserLoginByAuthId`;

CREATE PROCEDURE `PUserLoginByAuthId`(
    IN `_auth_id` VARBINARY(256),
    IN `_hash` BLOB
)
BEGIN
    DECLARE `_tmp_user_id` INT;
    DECLARE `_tmp_hash` BLOB;
    
    SELECT `user_id`
    INTO `_tmp_user_id`
    FROM `DAuthId` WHERE `auth_id` = `_auth_id`
    LIMIT 1;
    
    IF `_tmp_user_id` IS NULL THEN 
        SELECT 3 as `return_id`; -- user of this nickname not found
    ELSE 
		SELECT `hash` INTO `_tmp_hash`
        FROM `DAuthHash`
        WHERE `user_id` = `_tmp_user_id`;
        IF `_tmp_hash` != `_hash` THEN
            SELECT 1 as `return_id`; -- wrong password
        ELSE 
            SELECT 0 as `return_id`, `_tmp_user_id` as `user_id`;
        END IF;
    END IF;
END

CREATE TABLE `DGroupMember` (
    `group_id` int(11) NOT NULL AUTO_INCREMENT,
    `user_id` int(11) NOT NULL,
    `priv` int(11) NOT NULL,
    `date_modified` datetime NOT NULL DEFAULT CURRENT_TIMESTAMP,
    `date_created` datetime NOT NULL DEFAULT CURRENT_TIMESTAMP,
    `date_expired` datetime DEFAULT NULL,
    PRIMARY KEY (`group_id`,`user_id`)
);

DROP PROCEDURE IF EXISTS `PGroupCreate`;

CREATE PROCEDURE `PGroupCreate`(
    `_creator_user_id` INT
)
BEGIN
    INSERT INTO `DGroupMember` (`user_id`,`priv`)
    VALUES (`_creator_user_id`, 2);
    SELECT last_insert_id() as `group_id`;
END

DROP PROCEDURE IF EXISTS `PGroupTransferOwner`;

CREATE PROCEDURE `PGroupTransferOwner`(
    `_group_id` INT,
	`_user_id_src` INT,
	`_user_id_dst` INT
)
BEGIN
    DECLARE `op_priv` INT DEFAULT NULL;
    SELECT `priv` INTO `op_priv`
    FROM `DGroupMember`
    WHERE `group_id` = `_group_id` AND `user_id` = `_user_id_src`;
    IF `op_priv` IS NULL THEN
		SELECT 2 AS `return_id`; -- src user not found
    ELSEIF `op_priv` != 2 THEN
		SELECT 1 AS `return_id`; -- permission denied
    ELSEIF (SELECT `user_id` FROM `DGroupMember` 
		WHERE `group_id` = `_group_id` AND `user_id` = `_user_id_dst`) IS NULL THEN
		SELECT 3 AS `return_id`; -- dest user not found
	ELSE 
		START TRANSACTION;
			INSERT INTO `DGroupMember` (`group_id`,`user_id`,`priv`)
			VALUES (`_group_id`,`_user_id_src`, 0)
			ON DUPLICATE KEY UPDATE `priv` = 0, `date_expired` = NULL, 
				`date_modified` = CURRENT_TIMESTAMP();
			INSERT INTO `DGroupMember` (`group_id`,`user_id`,`priv`)
			VALUES (`_group_id`,`_user_id_dst`, 2)
			ON DUPLICATE KEY UPDATE `priv` = 2, `date_expired` = NULL, 
				`date_modified` = CURRENT_TIMESTAMP();
		COMMIT;
		SELECT 0 AS `return_id`; -- success
    END IF;
END

DROP PROCEDURE IF EXISTS `PGroupDelete`;

CREATE PROCEDURE `PGroupDelete`(
    `_group_id` INT,
    `_operator_user_id` INT
)
BEGIN
    DECLARE `op_priv` INT DEFAULT NULL;
    SELECT `priv` INTO `op_priv`
    FROM `DGroupMember`
    WHERE `group_id` = `_group_id` AND `user_id` = `_operator_user_id`
		AND (`date_expired` IS NULL OR `date_expired` > CURRENT_TIMESTAMP());
	IF `op_priv` IS NULL THEN 
		SELECT 2 as `return_id`; -- group or user not found
	ELSEIF `op_priv` != 2 THEN 
		SELECT 1 as `return_id`; -- permission denied
	ELSE 
		-- expire all group members
		UPDATE `DGroupMember` 
		SET `date_expired` = CURRENT_TIMESTAMP()
		WHERE `group_id` = `_group_id` AND `date_expired` IS NULL;
		SELECT 0 as `return_id`; -- success
	END IF;
END
