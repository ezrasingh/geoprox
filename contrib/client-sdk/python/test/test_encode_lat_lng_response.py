# coding: utf-8

"""
    geoprox-server

    No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)

    The version of the OpenAPI document: 0.1.0
    Generated by OpenAPI Generator (https://openapi-generator.tech)

    Do not edit the class manually.
"""  # noqa: E501


import unittest

from geoprox_client.models.encode_lat_lng_response import EncodeLatLngResponse

class TestEncodeLatLngResponse(unittest.TestCase):
    """EncodeLatLngResponse unit test stubs"""

    def setUp(self):
        pass

    def tearDown(self):
        pass

    def make_instance(self, include_optional) -> EncodeLatLngResponse:
        """Test EncodeLatLngResponse
            include_optional is a boolean, when False only required
            params are included, when True both required and
            optional params are included """
        # uncomment below to create an instance of `EncodeLatLngResponse`
        """
        model = EncodeLatLngResponse()
        if include_optional:
            return EncodeLatLngResponse(
                geohash = ''
            )
        else:
            return EncodeLatLngResponse(
                geohash = '',
        )
        """

    def testEncodeLatLngResponse(self):
        """Test EncodeLatLngResponse"""
        # inst_req_only = self.make_instance(include_optional=False)
        # inst_req_and_optional = self.make_instance(include_optional=True)

if __name__ == '__main__':
    unittest.main()