@Library('microservice@main') _ 

pipeline {
    agent any
    stages {
        stage("Load project configuration"){
            steps{
                script{
                    def projectConfig = readJSON file: 'config.json'
                        env.github_repo=projectConfig.github_repo
                    // env.service_name = projectConfig.serviceName
                    // env.notificationRecipients = projectConfig.notificationRecipients
                    // env.docker_username=projectConfig.docker_username
                    // env.kubernetes_endpoint=projectConfig.kubernetes_endpoint
                    // env.bucket_name=projectConfig.bucket_name 
                    // env.bucket_path=projectConfig.bucket_path   
                    // env.docker_credentials=projectConfig.docker_credentials
                    // env.docker_registry=projectConfig.docker_registry
                    // env.kubernetesClusterName=projectConfig.kubernetesClusterName
                    // env.kubernetesCredentialsId=projectConfig.kubernetesCredentialsId
                    // env.kubernetesCaCertificate=projectConfig.kubernetesCaCertificate
                    // env.gcp_credid=projectConfig.gcp_credid
                    // env.aws_credid=projectConfig.aws_credid
                }
            }
        }
        stage("Development Workflow") {
            agent any
            when {
                branch 'dev'
            }
            stages {
                stage("Clone Dev Repo & Get Version") {
                    steps {
                        script{
                            // Clone the dev branch
                            git branch: "${env.BRANCH_NAME}",url: "${env.github_repo}"
                            // git branch: 'dev',credentialsId: 'github-token',url: "https://github.com/ammohan6212/front-end.git"

                            // Fetch all tags
                            sh 'git fetch --tags'

                            // Get the latest tag correctly
                            def version = sh(
                                script: "git describe --tags \$(git rev-list --tags --max-count=1)",
                                returnStdout: true
                            ).trim()
                            env.version = version
                            echo "VERSION=${env.VERSION}"
                        }
                    }
                }
                stage("Detect Programming Language") {
                    steps {
                        detectLanguage() // Calls vars/detectLanguage.groovy
                    }
                }
                stage("Linting the Code and terraform linting and kubernetes linting and  docker linting") {
                    steps {
                        runLinter(env.DETECTED_LANG)
                        runInfrastructureLinting('terraform/')
                        runKubernetesLinting('kubernetes/') 
                        validateDockerImage('Dockerfile')
                    }
                }
                stage("YAML or JSON Schema Validation") {
                    steps {
                        // Example: Adjust to your specific YAML/JSON files and schemas
                        // performYamlJsonValidation('config.yaml', 'schemas/config_schema.json')
                        echo "Skipping general YAML/JSON validation (add specific calls here)."
                    }
                }
                stage("Secrets Detection") {
                    steps {
                        performSecretsDetection('.') // Scan the entire workspace
                    }
                }
                stage("Install Dependencies and dependency scanning and type checking and unit tests and code coverage calcualtion ") {
                    steps {
                        installAppDependencies(env.DETECTED_LANG)
                        performDependencyScan(env.DETECTED_LANG)
                        runTypeChecks(env.DETECTED_LANG)
                        runUnitTests(env.DETECTED_LANG)
                        calculateCodeCoverage(env.DETECTED_LANG)
                    }
                }
                stage("sonarqube and Mutation Testing and snapshot and component testing at Dev") {
                    steps {
                        runSonarQubeScan(env.SONAR_PROJECT_KEY)
                        runMutationTests(env.DETECTED_LANG)
                        runSnapshotTests(env.DETECTED_LANG)
                        runComponentTests(env.DETECTED_LANG)
                    }
                }
                stage("Building the Application") {
                    steps {
                        buildApplication(env.DETECTED_LANG)
                    }
                }
                stage("Create Archiving File and push the artifact ") {
                    steps {
                        createArchive("${env.service_name}-${env.BRANCH_NAME}-${env.version}.zip", 'src/')
                        pushArtifact("${env.service_name}-${env.version}-${env.BRANCH_NAME}.zip", "s3://${env.AWS_S3_BUCKET}/${env.AWS_S3_PATH}")
                    }
                }
                stage("Perform building and  docker linting Container Scanning using trivy and syft and docker scout and Dockle and snyk at Test Env") {
                    steps {
                        buildDockerImage("${env.docker_username}/${env.service_name}-${env.BRANCH_NAME}", env.version, '.')
                        validateDockerImage("${env.docker_username}/${env.service_name}-${env.BRANCH_NAME}:${env.version}")
                        validateDockerImage("${env.docker_username}/${env.service_name}-${env.BRANCH_NAME}:${env.version}")
                        scanContainerTrivy("${env.docker_username}/${env.service_name}-${env.BRANCH_NAME}:${env.version}")
                        scanContainerSyftDockle("${env.docker_username}/${env.service_name}-${env.BRANCH_NAME}:${env.version}")
                        scanContainerSnyk("${env.docker_username}/${env.service_name}-${env.BRANCH_NAME}:${env.version}", "Dockerfile")
                        scanContainerDockerScout("${env.docker_username}/${env.service_name}-${env.BRANCH_NAME}:${env.version}")
                        scanContainerGrype("${env.docker_username}/${env.service_name}-${env.BRANCH_NAME}:${env.version}")
                    }
                }
                stage("Perform Integration and ui/component testingand static security analysis and chaos testing with Docker Containers") {
                    steps {
                        integrationWithDocker()
                        runUiComponentTests(env.DETECTED_LANG)
                        performStaticSecurityAnalysis(env.DETECTED_LANG)
                        runChaosTests(env.DETECTED_LANG)
                    }
                }
                stage("Push Docker Image to dev env Registry") {
                    steps {
                        pushDockerImageToRegistry("${env.docker_registr}", "${env.docker_credentials}", "${env. DOCKER_USERNAME}${env.service_name}-${env.BRANCH_NAME}:${env.version}")
                    }
                }
                stage("Deploy to Dev") {
                    steps {
                       withKubeConfig(
                            caCertificate: env.kubernetesCaCertificate, // Now dynamic
                            clusterName: env.kubernetesClusterName,     // Now dynamic
                            contextName: '',
                            credentialsId: env.kubernetesCredentialsId, // Now dynamic
                            namespace: "${env.BRANCH_NAME}",
                            restrictKubeConfigAccess: false,
                            serverUrl: env.kubernetes_endpoint
                        ) {
                            // Change Kubernetes service selector to route traffic to Green
                            sh """kubectl apply -f blue-load.yml -n ${KUBE_NAMESPACE}"""
                        }
                    }
                }
                stage("Perform Smoke Testing and sanity testing and APi testing and integratio testing andlight ui test and regression testing feature flag and chaos and security After Dev Deploy") {
                    steps {
                        performSmokeTesting(env.DETECTED_LANG)
                        performSanityTesting(env.DETECTED_LANG)
                        performApiTesting(env.DETECTED_LANG)
                        performIntegrationTesting(env.DETECTED_LANG)
                        performLightUiTests(env.DETECTED_LANG)
                        performRegressionTesting(env.DETECTED_LANG)
                        performFeatureFlagChecks(env.DETECTED_LANG)
                        performSecurityChecks(env.DETECTED_LANG)
                        performChaosTestingAfterDeploy(env.DETECTED_LANG)
                        performLoadPerformanceTesting(env.DETECTED_LANG)
                    }
                }                
                stage("Perform Logging and Monitoring Checks After Dev Deploy") {
                    steps {
                        performLoggingMonitoringChecks()
                    }
                }
                stage("Need the manual approval to complete the dev env"){
                    steps{
                        sendEmailNotification('Alert', env.RECIPIENTS)
                    }
                }
                stage("Manual Approval for Dev Stage") {
                    steps {
                        input message: "Does everything working fine here", ok: "Deploy Now", submitter: "manager,admin"
                    }
                }
                stage("Generate Version File Dev Env") {
                    agent { label 'security-agent'} // Use a specific agent if needed
                    steps {
                        generateVersionFile('gcp', "${env.bucket_name}", "${gcp_credid}")

                    }
                }
            }
        }

        stage("Test Environment Workflow") {
            when {
                branch 'test'
            }
            stages {
                stage("send the alert mail to start the test env"){
                    steps{
                        sendEmailNotification('Alert', env.RECIPIENTS)
                    }
                }
                stage("Manual Approval to Start Test Env") {
                    steps {
                        input message: "Do you approve deployment to test?", ok: "Deploy Now", submitter: "manager,admin"
                    }
                }
                stage("Clone Repo with Test Branch & Get Version") {
                    steps {
                        script{
                            // Clone the dev branch
                            git branch: "${env.BRANCH_NAME}",url: "${env.github_repo}"
                            // git branch: 'dev',credentialsId: 'github-token',url: "https://github.com/ammohan6212/front-end.git"

                            // Fetch all tags
                            sh 'git fetch --tags'

                            // Get the latest tag correctly
                            def version = sh(
                                script: "git describe --tags \$(git rev-list --tags --max-count=1)",
                                returnStdout: true
                            ).trim()
                            env.version = version
                            echo "VERSION=${env.VERSION}"
                        }
                    }
                }
                stage("Static Code Analysis and unit tests and code coverage and dependencies and dependency check at Test") {
                    steps {
                        runUnitTests(env.DETECTED_LANG)
                        calculateCodeCoverage(env.DETECTED_LANG)
                        runSonarQubeScan(env.SONAR_PROJECT_KEY)
                        installAppDependencies(env.DETECTED_LANG)
                        performDependencyScan(env.DETECTED_LANG)
                    }
                }
                stage("Create Archiving File and push the artifact  at Test Stage") {
                    steps {
                        createArchive("${env.service_name}-${env.BRANCH_NAME}-${env.version}.zip", 'src/')
                        pushArtifact("${env.service_name}-${env.BRANCH_NAME}-${env.version}.zip", "s3://${env.AWS_S3_BUCKET}/${env.AWS_S3_PATH}")
                    }
                }
                stage("Perform building and  docker linting Container Scanning using trivy and syft and docker scout and Dockle and snyk at Test Env") {
                    steps {
                        buildDockerImage("${env.docker_username}/${env.service_name}-${env.BRANCH_NAME}", env.VERSION_TAG, '.')
                        validateDockerImage("${env.docker_username}/${env.service_name}-${env.BRANCH_NAME}:${env.version}")
                        scanContainerTrivy("${env.docker_username}/${env.service_name}-${env.BRANCH_NAME}:${env.version}")
                        scanContainerSyftDockle("${env.docker_username}/${env.service_name}-${env.BRANCH_NAME}:${env.version}")
                        scanContainerSnyk("${env.docker_username}/${env.service_name}-${env.BRANCH_NAME}:${env.version}", "Dockerfile")
                        scanContainerDockerScout("${env.docker_username}/${env.service_name}-${env.BRANCH_NAME}:${env.version}")
                        scanContainerGrype("${env.docker_username}/${env.service_name}-${env.BRANCH_NAME}:${env.version}")
                    }
                }
                stage("Push Docker Image to Preprod Registry") {
                    steps {
                        pushDockerImageToRegistry("${env.docker_registr}", "${env.docker_credentials}", "${env. DOCKER_USERNAME}${env.service_name}-${env.BRANCH_NAME}:${env.version}")
                    }
                }
                stage("Deploy to test") {
                    steps {
                        withKubeConfig(
                            caCertificate: env.kubernetesCaCertificate, // Now dynamic
                            clusterName: env.kubernetesClusterName,     // Now dynamic
                            contextName: '',
                            credentialsId: env.kubernetesCredentialsId, // Now dynamic
                            namespace: "${env.BRANCH_NAME}",
                            restrictKubeConfigAccess: false,
                            serverUrl: env.kubernetes_endpoint
                        ){
                            // Change Kubernetes service selector to route traffic to Green
                            sh """kubectl apply -f blue-load.yml -n ${KUBE_NAMESPACE}"""
                        }
                    }
                }
                stage("Smoke Test in Test Env") {
                    steps {
                        performSmokeTesting(env.DETECTED_LANG)
                    }
                }
                stage("Sanity Tests in Test Env") {
                    steps {
                        performSanityTesting(env.DETECTED_LANG)
                    }
                }
                stage("Full Integration Tests in Test Env") {
                    steps {
                        performIntegrationTesting(env.DETECTED_LANG)
                    }
                }
                stage("Functional Testing in Test Env") {
                    steps {
                        performApiTesting(env.DETECTED_LANG) // Reusing API testing for functional tests
                    }
                }
                stage("API Testing in Test Env") {
                    steps {
                        performApiTesting(env.DETECTED_LANG)
                    }
                }
                stage("Regression Testing in Test Env") {
                    steps {
                        performRegressionTesting(env.DETECTED_LANG)
                    }
                }
                stage("Database Testing in Test Env") {
                    steps {
                        performDatabaseTesting()
                    }
                }
                stage("Generate Version File Test Env") {
                    steps {
                        generateVersionFile('gcp', "${env.bucket_name}", "${gcp_credid}")
                    }
                }
                stage("Need the manual approval to complete the test env"){
                    steps{
                        sendEmailNotification('Alert', env.RECIPIENTS)
                    }
                }
                stage("Approval for Test Success") {
                    steps {
                        input message: "Do you approve to proceed to Staging Environment?", ok: "Approve", submitter: "manager,admin"
                    }
                }
            }
        }
        stage("deploying the application into prod"){
            when {
                branch 'main' // Or 'master'
            }
            stages{
                stage("Approval Before Deploying to Production") {
                    steps {
                        input message: "Do you approve deployment to Production?", ok: "Deploy Now", submitter: "manager,admin"
                    }
                }
                stage("create the change request containing what is changing and any DB changes and any downtime and rollback plan if deplyoment failes and deploymentwindow and stakeholders"){
                    steps{
                        script{
                            sh """"""
                        }
                    }
                }
                stage("Clone Repo with Main Branch & Get Version") {
                    steps {
                        script{
                            // Clone the dev branch
                            git branch: "${env.BRANCH_NAME}",url: "${env.github_repo}"
                            // git branch: 'dev',credentialsId: 'github-token',url: "https://github.com/ammohan6212/front-end.git"

                            // Fetch all tags
                            sh 'git fetch --tags'

                            // Get the latest tag correctly
                            def version = sh(
                                script: "git describe --tags \$(git rev-list --tags --max-count=1)",
                                returnStdout: true
                            ).trim()
                            env.version = version
                            echo "VERSION=${env.VERSION}"
                        }
                    }
                }
                stage("Detect Programming Language") {
                    steps {
                        detectLanguage() // Calls vars/detectLanguage.groovy
                    }
                }
                stage("Static Code Analysis and unit test and code coverage at Staging") {
                    steps {
                        runSonarQubeScan(env.SONAR_PROJECT_KEY)
                        runUnitTests(env.DETECTED_LANG)
                        calculateCodeCoverage(env.DETECTED_LANG)
                    }
                }
                stage("Install Dependencies and Scan Dependencies at Staging") {
                    steps {
                        installAppDependencies(env.DETECTED_LANG)
                        performDependencyScan(env.DETECTED_LANG)
                    }
                }
                stage("Create Archiving File and push the artifact at Staging Env") {
                    steps {
                        createArchive("${env.service_name}-${env.BRANCH_NAME}-${env.version}.zip", 'src/')
                        pushArtifact("${env.service_name}-${env.BRANCH_NAME}-${env.version}.zip", "s3://${env.AWS_S3_BUCKET}/${env.AWS_S3_PATH}")
                    }
                }
                stage("Perform build and   docker linting Container Scanning using trivy and syft and docker scout and Dockle and snyk at Test Env") {
                    steps {
                        buildDockerImage("${env.docker_username}/${env.service_name}-${env.BRANCH_NAME}:${env.version}", env.VERSION_TAG, '.')
                        validateDockerImage("${env.docker_username}/${env.service_name}-${env.BRANCH_NAME}:${env.version}")
                        scanContainerTrivy("${env.docker_username}/${env.service_name}-${env.BRANCH_NAME}:${env.version}")
                        scanContainerSyftDockle("${env.docker_username}/${env.service_name}-${env.BRANCH_NAME}:${env.version}")
                        scanContainerSnyk("${env.docker_username}/${env.service_name}-${env.BRANCH_NAME}:${env.version}", "Dockerfile")
                        scanContainerDockerScout("${env.docker_username}/${env.service_name}-${env.BRANCH_NAME}:${env.version}")
                        scanContainerGrype("${env.docker_username}/${env.service_name}-${env.BRANCH_NAME}:${env.version}")
                    }
                }
                stage("send the notification to CAB team to verify the deployment"){
                    steps{
                        sendEmailNotification('Alert', env.RECIPIENTS)
                    }
                }
                stage("need the CAB approvals before deplyign to the production"){
                    steps{
                        script{
                            input message: "Do you approve deployment to Production?", ok: "Deploy Now", submitter: "manager,admin"
                        }
                    }
                }
                
                stage("Need the manual approval from manager and stakeholders to deploy the application into prod"){
                    steps{
                        sendEmailNotification('Alert', env.RECIPIENTS)
                    }
                }
                stage("need approvals to next stage"){
                    steps{
                        script{
                            input message: "Do you approve deployment to Production?", ok: "Deploy Now", submitter: "manager,admin"
                        }
                    }
                }
                stage("Push Docker Image to stag Registry") {
                    steps {
                        pushDockerImageToRegistry("${env.docker_registr}", "${env.docker_credentials}", "${env. DOCKER_USERNAME}${env.service_name}-${env.BRANCH_NAME}:${env.version}")
                    }
                }
                stage("Deploy to prod at peak off -hours") {
                    steps {
                        withKubeConfig(
                            caCertificate: env.kubernetesCaCertificate, // Now dynamic
                            clusterName: env.kubernetesClusterName,     // Now dynamic
                            contextName: '',
                            credentialsId: env.kubernetesCredentialsId, // Now dynamic
                            namespace: "${env.BRANCH_NAME}",
                            restrictKubeConfigAccess: false,
                            serverUrl: env.kubernetes_endpoint
                        ){
                            // Change Kubernetes service selector to route traffic to Green
                            sh """kubectl apply -f blue-load.yml -n ${KUBE_NAMESPACE}"""
                        }
                    }
                }
                stage("Smoke Test in preProduction") {
                    steps {
                        performSmokeTesting(env.DETECTED_LANG)
                    }
                }
                stage("Sanity Test in preProduction") {
                    steps {
                        performSanityTesting(env.DETECTED_LANG)
                    }
                }
                stage("synthatic tests after prod  deployment"){
                    steps {
                        script {
                            sh """"""
                        }
                    }
                }
                stage("monitoring the prod environment"){
                    steps {
                        script {
                            sh """"""
                        }
                    }
                }
                stage("if prod fails rollback to the previous verion"){
                    steps { 
                        script {
                            sh """"""
                        }
                    }
                }
                stage("Generate Version File preprod Env") {
                    steps {
                        generateVersionFile('gcp', "${env.bucket_name}", "${gcp_credid}")
                    }
                }
                stage("Need the manual approval to deploy the application into prod"){
                    steps{
                        sendEmailNotification('Alert', env.RECIPIENTS)
                    }
                }
                stage("need approvals to complete the deployment"){
                    steps{
                        script{
                            input message: "Do you approve deployment to Production?", ok: "Deploy Now", submitter: "manager,admin"
                        }
                    }
                }
                stage("prod deployment is successful"){
                    steps{
                        script{
                            echo "the production deplyment successful"
                        }
                    }
                }



            }

        }
    }
    post {
        always {
            cleanWs() 
        }
        success {
            sendEmailNotification('SUCCESS', env.RECIPIENTS)
        }
        unstable {
            sendEmailNotification('UNSTABLE', env.RECIPIENTS)
        }
        failure {
            sendEmailNotification('FAILURE', env.RECIPIENTS)
        }
        aborted {
            sendEmailNotification('ABORTED', env.RECIPIENTS)
        }
    }
}