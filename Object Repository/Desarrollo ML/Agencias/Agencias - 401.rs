<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>Agencias - 401</name>
   <tag></tag>
   <elementGuidId>bbc87789-865f-4b1e-99d8-27be24089af3</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <authorizationRequest>
      <authorizationInfo>
         <entry>
            <key>bearerToken</key>
            <value>eyJhbGciOiJIUzUxMiJ9.eyJzdWIiOiIkMmEkMTAkZ2c1Sjc4RUxjcGhKYS8yc0tadGpJLjh1QllGbS9OMERyZ3BSaG53b1lQeXlPdDdVNVg5SzIiLCJleHAiOjE3MDU0NTUzMDMsIm5vbWJyZSI6IjMxOTkyMjgifQ.4ZXBDawGzi_mBHxzTPVO8AEzEXHzJTuPOGr2a96HptbdMgqd_ICcfuRc_6YPtj53MLZHVeiKUklaCQbDpQHTvw</value>
         </entry>
      </authorizationInfo>
      <authorizationType>Bearer</authorizationType>
   </authorizationRequest>
   <autoUpdateContent>false</autoUpdateContent>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent>{
  &quot;text&quot;: &quot;{\n    \&quot;country_id\&quot;: \&quot;PE\&quot;\n}&quot;,
  &quot;contentType&quot;: &quot;text/plain&quot;,
  &quot;charset&quot;: &quot;UTF-8&quot;
}</httpBodyContent>
   <httpBodyType>text</httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Content-Type</name>
      <type>Main</type>
      <value>text/plain</value>
      <webElementGuid>05dc2b58-77b7-4101-bff2-d042e459f06e</webElementGuid>
   </httpHeaderProperties>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer eyJhbGciOiJIUzUxMiJ9.eyJzdWIiOiIkMmEkMTAkZ2c1Sjc4RUxjcGhKYS8yc0tadGpJLjh1QllGbS9OMERyZ3BSaG53b1lQeXlPdDdVNVg5SzIiLCJleHAiOjE3MDU0NTUzMDMsIm5vbWJyZSI6IjMxOTkyMjgifQ.4ZXBDawGzi_mBHxzTPVO8AEzEXHzJTuPOGr2a96HptbdMgqd_ICcfuRc_6YPtj53MLZHVeiKUklaCQbDpQHTvw</value>
      <webElementGuid>d8f97ca0-eae3-4b27-915f-c36b5bb32899</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.6.8</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>POST</restRequestMethod>
   <restUrl>https://dev-apimp.olvacourier.com/agencies</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
