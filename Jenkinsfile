
pipeline {
    agent any
    options {
        skipStagesAfterUnstable()
    }
    stages {
         stage('Clone repository') { 
            steps { 
                script{
                checkout scm
                }
            }
        }

        stage('test and publish') { 
            steps { 
                script{
                 app = docker.build("underwater")
                }
            }
        }
    }
}

