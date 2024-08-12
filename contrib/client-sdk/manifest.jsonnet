local meta = {
  clientName: 'geoprox-client',
  clientVersion: '0.4.2',
  clientRepo: 'https://github.com/ezrasingh/geoprox/contrib/client-sdk',
  clientDescription: 'An HTTP client library for interacting with the Geoprox search engine',
  homePage: 'https://github.com/ezrasingh/geoprox/',
};

local snakeCase(label) = std.strReplace(label, '-', '_');

/**
OpenAPI generator config can be found here:
https://openapi-generator.tech/docs/generators/
*/
{
  'csharp.yaml': std.manifestYamlDoc(
    {
      packageName: 'GeoproxClient',
      packageVersion: meta.clientVersion,
      netCoreProjectFile: true,
    },
  ),
  'dart.yaml': std.manifestYamlDoc(
    {
      pubName: snakeCase(meta.clientName),
      pubLibrary: snakeCase(meta.clientName) + '.api',
      pubRepository: meta.clientRepo + '/dart/',
      pubDescription: meta.clientDescription,
      pubVersion: meta.clientVersion,
      pubHomepage: meta.homePage,
    },
  ),
  'dart-dio.yaml': std.manifestYamlDoc(
    {
      pubName: snakeCase(meta.clientName) + '_dio',
      pubLibrary: snakeCase(meta.clientName) + '_dio.api',
      pubRepository: meta.clientRepo + '/dart-dio/',
      pubDescription: meta.clientDescription,
      pubVersion: meta.clientVersion,
      pubHomepage: meta.homePage,
    },
  ),
  'elixir.yaml': std.manifestYamlDoc(
    {
      packageName: snakeCase(meta.clientName),

    },
  ),
  'go.yaml': std.manifestYamlDoc(
    {
      packageName: snakeCase(meta.clientName),
      packageVersion: meta.clientVersion,
    },
  ),
  'groovy.yaml': std.manifestYamlDoc(
    {
      artifactId: meta.clientName + '-groovy',
      artifactVersion: meta.clientVersion,
      groupId: 'org.geoprox.groovy',
      apiPackage: 'org.geoprox.groovy.client',
      invokerPackage: 'org.geoprox.groovy.client',
      modelPackage: 'org.geoprox.groovy.client.model',
      developerOrganizationUrl: meta.homePage,
      scmUrl: meta.clientRepo + '/groovy/',
    },
  ),
  'kotlin.yaml': std.manifestYamlDoc(
    {
      artifactId: meta.clientName,
      artifactVersion: meta.clientVersion,
      groupId: 'org.geoprox',
      packageName: 'org.geoprox.client',
    },
  ),
  'php.yaml': std.manifestYamlDoc(
    {
      packageName: 'GeoproxClient',
      composerPackageName: meta.clientName,
      artifactVersion: meta.clientVersion,
      artifactUrl: meta.clientRepo + '/php/',
      developerOrganizationUrl: meta.homePage,
    },
  ),
  'python.yaml': std.manifestYamlDoc(
    {
      packageName: snakeCase(meta.clientName),
      packageVersion: meta.clientVersion,
      packageUrl: meta.clientRepo + '/python/',
      projectName: meta.clientName,
    },
  ),
  'ruby.yaml': std.manifestYamlDoc(
    {
      gemName: snakeCase(meta.clientName),
      gemVersion: meta.clientVersion,
      gemHomepage: meta.clientRepo + '/ruby/',
    },
  ),
  'rust.yaml': std.manifestYamlDoc(
    {
      packageName: meta.clientName,
      packageVersion: meta.clientVersion,
      hideGenerationTimestamp: false,
    },
  ),
  'typescript-axios.yaml': std.manifestYamlDoc({
    npmName: meta.clientName + '-ts-axios',
    npmVersion: meta.clientVersion,
    npmRepository: meta.clientRepo + '/typescript-axios/',
  }),
  'typescript-fetch.yaml': std.manifestYamlDoc({
    npmName: meta.clientName + '-ts-fetch',
    npmVersion: meta.clientVersion,
    npmRepository: meta.clientRepo + '/typescript-fetch/',
  }),
  'typescript-node.yaml': std.manifestYamlDoc({
    npmName: meta.clientName + '-ts-node',
    npmVersion: meta.clientVersion,
    npmRepository: meta.clientRepo + '/typescript-node/',
  }),
  'typescript-rxjs.yaml': std.manifestYamlDoc({
    npmName: meta.clientName + '-ts-rxjs',
    npmVersion: meta.clientVersion,
    npmRepository: meta.clientRepo + '/typescript-rxjs/',
  }),
}
