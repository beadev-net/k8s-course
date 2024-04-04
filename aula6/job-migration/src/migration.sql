-- Migração para criar a tabela 'surveys'
CREATE TABLE surveys (
    id INT AUTO_INCREMENT PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    choices JSON NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

-- Migração para criar a tabela 'votes'
CREATE TABLE votes (
    id INT AUTO_INCREMENT PRIMARY KEY,
    survey_id INT NOT NULL,
    vote VARCHAR(255) NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (survey_id) REFERENCES surveys(id)
);

INSERT INTO surveys (name, choices) VALUES ('Meu Primeiro Survey', '["Congrats", "Good Job", "Keep Going"]');