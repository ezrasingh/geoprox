# coding: utf-8

"""
    geoprox-server

    No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)

    The version of the OpenAPI document: 0.1.0
    Generated by OpenAPI Generator (https://openapi-generator.tech)

    Do not edit the class manually.
"""  # noqa: E501


import unittest

from geoprox_client.models.query_range_response import QueryRangeResponse

class TestQueryRangeResponse(unittest.TestCase):
    """QueryRangeResponse unit test stubs"""

    def setUp(self):
        pass

    def tearDown(self):
        pass

    def make_instance(self, include_optional) -> QueryRangeResponse:
        """Test QueryRangeResponse
            include_optional is a boolean, when False only required
            params are included, when True both required and
            optional params are included """
        # uncomment below to create an instance of `QueryRangeResponse`
        """
        model = QueryRangeResponse()
        if include_optional:
            return QueryRangeResponse(
                found = [
                    geoprox_client.models.keys_found_inner.KeysFound_inner(
                        distance = 1.337, 
                        key = '', )
                    ]
            )
        else:
            return QueryRangeResponse(
                found = [
                    geoprox_client.models.keys_found_inner.KeysFound_inner(
                        distance = 1.337, 
                        key = '', )
                    ],
        )
        """

    def testQueryRangeResponse(self):
        """Test QueryRangeResponse"""
        # inst_req_only = self.make_instance(include_optional=False)
        # inst_req_and_optional = self.make_instance(include_optional=True)

if __name__ == '__main__':
    unittest.main()
