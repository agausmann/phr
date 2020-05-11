ALTER TABLE race_entrants
    MODIFY COLUMN reason ENUM('dns', 'dnf', 'dsq');
