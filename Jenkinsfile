pipeline {
    agent {
        docker {
            image 'rust:1-buster'
        }
    }

    stages {

        stage('Install clippy') {
            steps {
                sh "rustup component add clippy"
            }
        }

        stage('Install code formatter') {
            steps {
                sh "rustup component add rustfmt"
            }
        }

        stage('Build') {
            steps {
                sh "cargo build"
            }
        }

        stage('Test') {
            steps {
                sh "cargo clippy --all"
            }
        }
    }
}
