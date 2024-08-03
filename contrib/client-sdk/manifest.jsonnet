local meta = {
  clientName: 'geoprox-client',
  clientVersion: '0.2.0',
  clientRepo: 'https://github.com/ezrasingh/geoprox/contrib/client-sdk',
};

/**
OpenAPI generator config can be found here:
https://openapi-generator.tech/docs/generators/
*/
{
  'python.yaml': std.manifestYamlDoc(
    {
      packageName: std.strReplace(meta.clientName, '-', '_'),
      packageVersion: meta.clientVersion,
      packageUrl: meta.clientRepo + '/python/',
      projectName: meta.clientName,
    },
  ),
  'ruby.yaml': std.manifestYamlDoc(
    {
      gemName: std.strReplace(meta.clientName, '-', '_'),
      gemVersion: meta.clientVersion,
      gemHomepage: meta.clientRepo + '/ruby/',
    },
  ),
  'rust.yaml': std.manifestYamlDoc(
    {
      packageName: meta.clientName,
      packageVersion: meta.clientVersion,
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
