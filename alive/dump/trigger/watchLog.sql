CREATE TRIGGER `watchLog` AFTER UPDATE ON `watch` FOR EACH ROW BEGIN
   IF OLD.err = 0 AND NEW.err != 0 THEN
      INSERT INTO log (watch_id,state,ts) VALUES (NEW.id,1,UNIX_TIMESTAMP());
   ELSEIF OLD.err != 0 AND NEW.err = 0 THEN
      INSERT INTO log (watch_id,state,ts) VALUES (NEW.id,0,UNIX_TIMESTAMP());
   END IF;
END ;;