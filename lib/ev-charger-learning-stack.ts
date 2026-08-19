import { HttpApi } from 'aws-cdk-lib/aws-apigatewayv2';
import { HttpLambdaIntegration } from 'aws-cdk-lib/aws-apigatewayv2-integrations';
import { HttpMethod } from 'aws-cdk-lib/aws-events';
import * as cdk from 'aws-cdk-lib/core';
import { RustFunction } from 'cargo-lambda-cdk';
import { Construct } from 'constructs';
// import * as sqs from 'aws-cdk-lib/aws-sqs';

export class EvChargerLearningStack extends cdk.Stack {
  constructor(scope: Construct, id: string, props?: cdk.StackProps) {
    super(scope, id, props);

    const availabilityFunction = new RustFunction(this, 'availabilityFunction', {
      manifestPath: './lambda/availability',
      runtime: 'provided.al2023',
      timeout: cdk.Duration.seconds(30),
    });

    const api = new HttpApi(this, 'availabilityApi');
    const availabilityInteg = new HttpLambdaIntegration('availabilityIntegration', availabilityFunction);

    api.addRoutes({
      path: '/availability',
      methods: [HttpMethod.GET],
      integration: availabilityInteg,
    })
    new cdk.CfnOutput(this, 'apiUrl', {
      description: 'The URL of the API Gateway',
      value: `https://${api.apiId}.execute-api.${this.region}.amazonaws.com`,
    })
  }
}
