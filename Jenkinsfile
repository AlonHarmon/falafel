
pipeline {
    agent any
    options {
        skipStagesAfterUnstable()
    }
    stages {
        stage('Initialize'){
            def dockerHome = tool 'myDocker'
            env.PATH = "${dockerHome}/bin:${env.PATH}"
        }
         stage('Clone repository') { 
            steps { 
                script{
                checkout scm
                }
            }
        }

        stage('test and publish') { 
            steps { 
                sh "docker build -t test --build-arg crates_token=${CRATES_TOKEN}"
            }
        }
    }
}

