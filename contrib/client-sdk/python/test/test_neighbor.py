# coding: utf-8

"""
    geoprox-server

    Geoprox server implementation providing a HTTP API for geospatial queries and position tracking

    The version of the OpenAPI document: 0.2.0
    Contact: singhezra@gmail.com
    Generated by OpenAPI Generator (https://openapi-generator.tech)

    Do not edit the class manually.
"""  # noqa: E501


import unittest

from geoprox_client.models.neighbor import Neighbor

class TestNeighbor(unittest.TestCase):
    """Neighbor unit test stubs"""

    def setUp(self):
        pass

    def tearDown(self):
        pass

    def make_instance(self, include_optional) -> Neighbor:
        """Test Neighbor
            include_optional is a boolean, when False only required
            params are included, when True both required and
            optional params are included """
        # uncomment below to create an instance of `Neighbor`
        """
        model = Neighbor()
        if include_optional:
            return Neighbor(
                distance = 1.337,
                key = ''
            )
        else:
            return Neighbor(
                distance = 1.337,
                key = '',
        )
        """

    def testNeighbor(self):
        """Test Neighbor"""
        # inst_req_only = self.make_instance(include_optional=False)
        # inst_req_and_optional = self.make_instance(include_optional=True)

if __name__ == '__main__':
    unittest.main()
