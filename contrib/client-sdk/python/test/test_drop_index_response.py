# coding: utf-8

"""
    geoprox-server

    No description provided (generated by Openapi Generator https://github.com/openapitools/openapi-generator)

    The version of the OpenAPI document: 0.1.0
    Generated by OpenAPI Generator (https://openapi-generator.tech)

    Do not edit the class manually.
"""  # noqa: E501


import unittest

from geoprox_client.models.drop_index_response import DropIndexResponse

class TestDropIndexResponse(unittest.TestCase):
    """DropIndexResponse unit test stubs"""

    def setUp(self):
        pass

    def tearDown(self):
        pass

    def make_instance(self, include_optional) -> DropIndexResponse:
        """Test DropIndexResponse
            include_optional is a boolean, when False only required
            params are included, when True both required and
            optional params are included """
        # uncomment below to create an instance of `DropIndexResponse`
        """
        model = DropIndexResponse()
        if include_optional:
            return DropIndexResponse(
                deleted = True
            )
        else:
            return DropIndexResponse(
                deleted = True,
        )
        """

    def testDropIndexResponse(self):
        """Test DropIndexResponse"""
        # inst_req_only = self.make_instance(include_optional=False)
        # inst_req_and_optional = self.make_instance(include_optional=True)

if __name__ == '__main__':
    unittest.main()
